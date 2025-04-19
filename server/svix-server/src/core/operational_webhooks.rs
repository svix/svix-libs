// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Module defining an interface for sending webhook events about the service.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;
use svix::api::{MessageIn, Svix, SvixOptions};

use super::{
    security::generate_management_token,
    types::{
        ApplicationId, ApplicationUid, EndpointId, EndpointUid, MessageAttemptId, MessageId,
        MessageUid, OrganizationId,
    },
};
use crate::{
    core::security::JwtSigningConfig,
    db::models::{endpoint, messageattempt},
    error::{Error, HttpError, Result},
};

/// Sent when an endpoint has been automatically disabled after continuous failures.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDisabledEventData {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub endpoint_id: EndpointId,
    pub endpoint_uid: Option<EndpointUid>,
    pub fail_since: DateTime<Utc>,
}

/// Sent when an endpoint is created, updated, or deleted
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub endpoint_id: EndpointId,
    pub endpoint_uid: Option<EndpointUid>,
}

impl EndpointEvent {
    pub fn new(app_uid: Option<&ApplicationUid>, endp: &endpoint::Model) -> Self {
        Self {
            app_id: endp.app_id.clone(),
            app_uid: app_uid.cloned(),
            endpoint_id: endp.id.clone(),
            endpoint_uid: endp.uid.clone(),
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttempetLast {
    pub id: MessageAttemptId,
    pub response_status_code: i16,
    pub timestamp: DateTime<Utc>,
}

impl From<messageattempt::Model> for MessageAttempetLast {
    fn from(attempt: messageattempt::Model) -> Self {
        Self {
            id: attempt.id,
            response_status_code: attempt.response_status_code,
            timestamp: attempt.created_at.into(),
        }
    }
}

/// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a
/// "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing"
/// event.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttemptEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub msg_id: MessageId,
    pub msg_event_id: Option<MessageUid>,
    pub endpoint_id: EndpointId,
    pub last_attempt: MessageAttempetLast,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum OperationalWebhook {
    #[serde(rename = "endpoint.disabled")]
    EndpointDisabled(EndpointDisabledEventData),
    #[serde(rename = "endpoint.created")]
    EndpointCreated(EndpointEvent),
    #[serde(rename = "endpoint.updated")]
    EndpointUpdated(EndpointEvent),
    #[serde(rename = "endpoint.deleted")]
    EndpointDeleted(EndpointEvent),
    #[serde(rename = "message.attempt.exhausted")]
    MessageAttemptExhausted(MessageAttemptEvent),
    #[serde(rename = "message.attempt.failing")]
    MessageAttemptFailing(MessageAttemptEvent),
    #[serde(rename = "message.attempt.recovered")]
    MessageAttemptRecovered(MessageAttemptEvent),
}

pub type OperationalWebhookSender = Arc<OperationalWebhookSenderInner>;

pub struct OperationalWebhookSenderInner {
    signing_config: Arc<JwtSigningConfig>,
    url: Option<String>,
}

impl OperationalWebhookSenderInner {
    pub fn new(keys: Arc<JwtSigningConfig>, url: Option<String>) -> Arc<Self> {
        // Sanitize the URL if present
        let sanitized_url = url.as_ref().map(|url_str| {
            // Remove trailing slashes
            let mut cleaned_url = url_str.trim().to_string();
            while cleaned_url.ends_with('/') {
                cleaned_url.pop();
            }
            
            // Return the cleaned URL or original if empty
            if !cleaned_url.is_empty() {
                cleaned_url
            } else {
                url_str.clone()
            }
        });
        
        Arc::new(Self {
            signing_config: keys,
            url: sanitized_url,
        })
    }

    pub async fn send_operational_webhook(
        &self,
        recipient_org_id: &OrganizationId,
        payload: OperationalWebhook,
    ) -> Result<()> {
        let Some(url) = &self.url else { return Ok(()) };

        let op_webhook_token =
            generate_management_token(&self.signing_config).map_err(Error::generic)?;
        let svix_api = Svix::new(
            op_webhook_token,
            Some(SvixOptions {
                server_url: Some(url.to_string()),
                ..Default::default()
            }),
        );

        let payload = serde_json::to_value(payload)
            .map_err(|_| HttpError::internal_server_error(None, None))?;

        // Get the event type from the type field
        let event_type: String = payload
            .get("type")
            .ok_or_else(|| HttpError::internal_server_error(None, None))?
            .as_str()
            .ok_or_else(|| HttpError::internal_server_error(None, None))?
            .to_string();

        let recipient_org_id = recipient_org_id.to_string();
        let url_clone = url.clone();  // Clone for use in the async block

        tokio::spawn(async move {
            // This sends a webhook under the Svix management organization. This organization contains
            // applications which are each a regular organization. The recipient's OrganizationId is the
            // app UID to use.
            let resp = svix_api
                .message()
                .create(
                    recipient_org_id.clone(),
                    MessageIn {
                        event_type,
                        payload,
                        ..MessageIn::default()
                    },
                    None,
                )
                .await;

            match resp {
                Ok(_) => {}
                // Handle 404s with more context
                Err(svix::error::Error::Http(svix::error::HttpErrorContent {
                    status: StatusCode::NOT_FOUND,
                    ..
                })) => {
                    // Try to determine if it's a connection issue or an app not found issue
                    if let Some(app_resp) = svix_api.application().get(recipient_org_id.clone(), None).await.ok() {
                        // App exists but endpoint not found
                        tracing::warn!(
                            "Operational webhooks are enabled, but no endpoint is configured for organization {} (app exists)",
                            recipient_org_id,
                        );
                    } else {
                        // App doesn't exist
                        tracing::warn!(
                            "Operational webhooks are enabled, but no application exists for organization {}",
                            recipient_org_id,
                        );
                    }
                }
                // Add specific handling for connection errors
                Err(svix::error::Error::Http(svix::error::HttpErrorContent {
                    status: StatusCode::BAD_REQUEST,
                    ..
                })) => {
                    tracing::error!(
                        "Failed sending operational webhook: Bad request. Check URL format: {}",
                        url_clone,
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Failed sending operational webhook for {} to URL {}: {}",
                        recipient_org_id,
                        url_clone,
                        e.to_string()
                    );
                }
            }
        });

        Ok(())
    }
}
