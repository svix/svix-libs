/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use crate::Configuration;
use crate::models;
use crate::error::Error;
#[allow(unused_imports)]
use crate::models::*;

/// struct for passing parameters to the method [`v1_period_inbound_period_msg`]
#[derive(Clone, Debug)]
pub struct V1PeriodInboundPeriodMsgParams {
    /// The app's ID or UID
    pub app_id: String,
    pub inbound_token: String,
    pub body: String,
    /// The event type's name
    pub event_type: Option<String>,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_inbound_period_rotate_url`]
#[derive(Clone, Debug)]
pub struct V1PeriodInboundPeriodRotateUrlParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}


/// struct for typed errors of method [`v1_period_inbound_period_msg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodInboundPeriodMsgError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_inbound_period_rotate_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodInboundPeriodRotateUrlError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}


/// Handles a raw inbound webhook for the application.
pub async fn v1_period_inbound_period_msg(configuration: &Configuration, params: V1PeriodInboundPeriodMsgParams) -> Result<models::MessageOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let inbound_token = params.inbound_token;
    let body = params.body;
    let event_type = params.event_type;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/app/{app_id}/inbound/msg/{inbound_token}")
    ;
    if let Some(ref s) = event_type {
        let query_value = s.to_string();
        req = req.with_query_param("event_type", query_value);
    }
    req = req.with_path_param("app_id", app_id.to_string());
    req = req.with_path_param("inbound_token", inbound_token.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key", param_value.to_string());
    }
    req = req.with_body_param(body);

    req.execute(configuration).await
}

/// Invalidates the previous inbound url (if one exists), producing a new inbound URL for this app.
pub async fn v1_period_inbound_period_rotate_url(configuration: &Configuration, params: V1PeriodInboundPeriodRotateUrlParams) -> Result<models::RotatedUrlOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/app/{app_id}/inbound/rotate-url")
    ;
    req = req.with_path_param("app_id", app_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key", param_value.to_string());
    }

    req.execute(configuration).await
}

