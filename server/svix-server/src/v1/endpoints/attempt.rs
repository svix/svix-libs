// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationId, ApplicationIdOrUid, BaseId, EndpointId, EndpointIdOrUid, EventChannel,
            EventTypeNameSet, MessageAttemptId, MessageAttemptTriggerType, MessageId,
            MessageIdOrUid, MessageStatus, StatusCodeClass,
        },
    },
    db::models::{endpoint, message, messagedestination},
    error::{Error, HttpError, Result},
    queue::{MessageTask, TaskQueueProducer},
    v1::{
        endpoints::message::MessageOut,
        utils::{
            api_not_implemented, EmptyResponse, ListResponse, MessageListFetchOptions, ModelOut,
            ValidatedQuery,
        },
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};

use hyper::StatusCode;
use sea_orm::{entity::prelude::*, DatabaseConnection, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use svix_server_derive::ModelOut;
use validator::Validate;

use crate::db::models::messageattempt;
use crate::v1::utils::Pagination;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
struct MessageAttemptOut {
    response: String,
    response_status_code: i16,
    status: MessageStatus,
    trigger_type: MessageAttemptTriggerType,
    msg_id: MessageId,
    endpoint_id: EndpointId,

    id: MessageAttemptId,

    #[serde(rename = "timestamp")]
    created_at: DateTime<Utc>,
}

// FIXME: This can and should be a derive macro
impl From<messageattempt::Model> for MessageAttemptOut {
    fn from(model: messageattempt::Model) -> Self {
        Self {
            response: model.response,
            response_status_code: model.response_status_code,
            status: model.status,
            trigger_type: model.trigger_type,
            msg_id: model.msg_id,
            endpoint_id: model.endp_id,

            id: model.id,
            created_at: model.created_at.into(),
        }
    }
}

/// A model containing information on a given message plus additional fields on the last attempt for
/// that message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttemptedMessageOut {
    #[serde(flatten)]
    msg: MessageOut,
    status: MessageStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_attempt: Option<DateTimeWithTimeZone>,
}

impl ModelOut for AttemptedMessageOut {
    fn id_copy(&self) -> String {
        self.msg.id.0.clone()
    }
}

impl AttemptedMessageOut {
    pub fn from_dest_and_msg(
        dest: messagedestination::Model,
        msg: message::Model,
    ) -> AttemptedMessageOut {
        AttemptedMessageOut {
            msg: msg.into(),
            status: dest.status,
            next_attempt: dest.next_attempt,
        }
    }
}

/// Additional parameters (besides pagination) in the query string for the "List Attempted Messages"
/// enpoint.
#[derive(Debug, Deserialize, Validate)]
pub struct ListAttemptedMessagesQueryParameters {
    #[validate]
    channel: Option<EventChannel>,
    status: Option<MessageStatus>,
}

/// Fetches a list of [`AttemptedMessageOut`]s associated with a given app and endpoint.
async fn list_attempted_messages(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(mut pagination): ValidatedQuery<Pagination<MessageId>>,
    ValidatedQuery(ListAttemptedMessagesQueryParameters { channel, status }): ValidatedQuery<
        ListAttemptedMessagesQueryParameters,
    >,
    Path((_app_id, endp_id)): Path<(ApplicationId, EndpointId)>,
    AuthenticatedApplication {
        permissions: _,
        app: _,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<AttemptedMessageOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.take();

    let mut dests_and_msgs = messagedestination::Entity::secure_find_by_endpoint(endp_id)
        .find_also_related(message::Entity)
        .order_by_desc(messagedestination::Column::MsgId)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        dests_and_msgs = dests_and_msgs.filter(messagedestination::Column::MsgId.lt(iterator));
    }

    if let Some(channel) = channel {
        dests_and_msgs = dests_and_msgs.filter(message::Column::Channels.contains(&channel));
    }

    if let Some(status) = status {
        dests_and_msgs = dests_and_msgs.filter(messagedestination::Column::Status.eq(status));
    }

    Ok(Json(AttemptedMessageOut::list_response(
        dests_and_msgs
            .all(db)
            .await?
            .into_iter()
            .map(
                |(dest, msg): (messagedestination::Model, Option<message::Model>)| {
                    let msg = msg.ok_or_else(|| {
                        Error::Database("No associated message with messagedestination".to_owned())
                    })?;
                    Ok(AttemptedMessageOut::from_dest_and_msg(dest, msg))
                },
            )
            .collect::<Result<_>>()?,
        limit as usize,
    )))
}

/// Additional parameters (besides pagination) in the query string for the "List Attempts by
/// Endpoint" enpoint.
#[derive(Debug, Deserialize, Validate)]
pub struct ListAttemptsByEndpointQueryParameters {
    status: Option<MessageStatus>,
    status_code_class: Option<StatusCodeClass>,
    #[validate]
    event_types: Option<EventTypeNameSet>,
    #[validate]
    channel: Option<EventChannel>,
}

/// Fetches a list of [`MessageAttemptOut`]s for a given endpoint ID
async fn list_attempts_by_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(mut pagination): ValidatedQuery<Pagination<MessageAttemptId>>,
    ValidatedQuery(ListAttemptsByEndpointQueryParameters {
        status,
        status_code_class,
        event_types,
        channel,
    }): ValidatedQuery<ListAttemptsByEndpointQueryParameters>,
    Path((_app_id, endp_id)): Path<(ApplicationId, EndpointId)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.take();

    // Confirm endpoint ID belongs to the given application
    if endpoint::Entity::secure_find_by_id(app.id, endp_id.clone())
        .one(db)
        .await?
        .is_none()
    {
        return Err(Error::Http(HttpError::not_found(None, None)));
    }

    let mut query = messageattempt::Entity::secure_find_by_endpoint(endp_id)
        .order_by_desc(messageattempt::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(messageattempt::Column::Id.lt(iterator));
    }

    if let Some(status) = status {
        query = query.filter(messageattempt::Column::Status.eq(status));
    }

    query = match status_code_class {
        Some(StatusCodeClass::CodeNone) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(0, 99))
        }

        Some(StatusCodeClass::Code1xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(100, 199))
        }

        Some(StatusCodeClass::Code2xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(200, 299))
        }

        Some(StatusCodeClass::Code3xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(300, 399))
        }

        Some(StatusCodeClass::Code4xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(400, 499))
        }

        Some(StatusCodeClass::Code5xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(500, 599))
        }

        None => query,
    };

    // The event_types and channel filter require joining the associated message
    if event_types.is_some() || channel.is_some() {
        query = query.join_rev(
            sea_orm::JoinType::InnerJoin,
            messageattempt::Entity::belongs_to(message::Entity)
                .from(messageattempt::Column::MsgId)
                .to(message::Column::Id)
                .into(),
        );

        if let Some(EventTypeNameSet(event_types)) = event_types {
            query = query.filter(message::Column::EventType.is_in(event_types))
        }

        if let Some(channel) = channel {
            query = query.filter(message::Column::Channels.contains(&channel))
        }
    }

    Ok(Json(MessageAttemptOut::list_response(
        query.all(db).await?.into_iter().map(Into::into).collect(),
        limit as usize,
    )))
}

#[derive(Debug, Deserialize, Validate)]
pub struct AttemptListFetchOptions {
    pub endpoint_id: Option<EndpointIdOrUid>,
    #[validate]
    pub channel: Option<EventChannel>,
    pub status: Option<MessageStatus>,
}

// FIXME: add filtering by event_types/channel
async fn list_messageattempts(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<MessageAttemptId>>,
    ValidatedQuery(AttemptListFetchOptions {
        endpoint_id,
        channel: _,
        status,
    }): ValidatedQuery<AttemptListFetchOptions>,
    list_filter: MessageListFetchOptions,
    Path((_app_id, msg_id)): Path<(ApplicationIdOrUid, MessageIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let limit = pagination.limit;
    let iterator = pagination
        .iterator
        .clone()
        .or_else(|| list_filter.before.map(MessageAttemptId::start_id));

    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut query = messageattempt::Entity::secure_find_by_msg(msg.id)
        .order_by_desc(messageattempt::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(messageattempt::Column::Id.lt(iterator))
    }

    if let Some(endpoint_id) = endpoint_id {
        let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
            .one(db)
            .await?
            .ok_or_else(|| HttpError::not_found(None, None))?;
        query = query.filter(messageattempt::Column::EndpId.eq(endp.id))
    }

    if let Some(status) = status {
        query = query.filter(messageattempt::Column::Status.eq(status))
    }

    Ok(Json(MessageAttemptOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

async fn get_messageattempt(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, msg_id, attempt_id)): Path<(
        ApplicationIdOrUid,
        MessageIdOrUid,
        MessageAttemptId,
    )>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<MessageAttemptOut>> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let attempt = messageattempt::Entity::secure_find_by_msg(msg.id)
        .filter(messageattempt::Column::Id.eq(attempt_id))
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(attempt.into()))
}

async fn resend_webhook(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Path((_app_id, msg_id, endp_id)): Path<(ApplicationIdOrUid, MessageIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    // Fetch it to make sure it was even a combination
    let _msg_dest = messagedestination::Entity::secure_find_by_msg(msg.id.clone())
        .filter(messagedestination::Column::EndpId.eq(endp.id.clone()))
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    queue_tx
        .send(
            MessageTask::new_task(
                msg.id.clone(),
                app.id,
                endp.id,
                MessageAttemptTriggerType::Manual,
            ),
            None,
        )
        .await?;
    Ok((StatusCode::ACCEPTED, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new().nest(
        "/app/:app_id/",
        Router::new()
            .nest(
                "msg/:msg_id",
                Router::new()
                    .route("/attempt/", get(list_messageattempts))
                    .route("/attempt/:attempt_id/", get(get_messageattempt))
                    .route("/endpoint/", get(api_not_implemented))
                    .route("/endpoint/:endp_id/resend/", post(resend_webhook))
                    .route("/endpoint/:endp_id/attempt/", get(api_not_implemented)),
            )
            .route("endpoint/:endp_id/msg/", get(list_attempted_messages))
            .route("attempt/endpoint/:endp_id/", get(list_attempts_by_endpoint))
            .route("attempt/msg/:msg_id/", get(api_not_implemented)),
    )
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use super::{AttemptedMessageOut, MessageAttemptOut};
    use crate::{
        test_util::{run_with_retries, start_svix_server, TestReceiver},
        v1::{
            endpoints::{
                application::tests::create_test_app, endpoint::tests::create_test_endpoint,
                message::tests::create_test_message,
            },
            utils::ListResponse,
        },
    };

    #[tokio::test]
    #[cfg_attr(not(feature = "integration_testing"), ignore)]
    async fn test_list_attempted_messages() {
        let (client, _jh) = start_svix_server();

        let app_id = create_test_app(&client, "v1AttemptListAttemptedMessagesTestApp")
            .await
            .unwrap();

        let receiver_1 = TestReceiver::start(axum::http::StatusCode::OK);
        let receiver_2 = TestReceiver::start(axum::http::StatusCode::OK);

        let endp_id_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
            .await
            .unwrap();
        let endp_id_2 = create_test_endpoint(&client, &app_id, &receiver_2.endpoint)
            .await
            .unwrap();

        let msg_1 = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
            .await
            .unwrap();
        let msg_2 = create_test_message(&client, &app_id, serde_json::json!({"test": "data2"}))
            .await
            .unwrap();
        let msg_3 = create_test_message(&client, &app_id, serde_json::json!({"test": "data3"}))
            .await
            .unwrap();

        let list_1: ListResponse<AttemptedMessageOut> = client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/msg/", app_id, endp_id_1),
                StatusCode::OK,
            )
            .await
            .unwrap();
        let list_2: ListResponse<AttemptedMessageOut> = client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/msg/", app_id, endp_id_2),
                StatusCode::OK,
            )
            .await
            .unwrap();

        for list in [list_1, list_2] {
            assert_eq!(list.data.len(), 3);

            // Assert order
            assert_eq!(list.data[0].msg.id, msg_3);
            assert_eq!(list.data[1].msg.id, msg_2);
            assert_eq!(list.data[2].msg.id, msg_1);

            let message_ids: Vec<_> = list.data.into_iter().map(|amo| amo.msg.id).collect();
            assert!(message_ids.contains(&msg_1));
            assert!(message_ids.contains(&msg_2));
            assert!(message_ids.contains(&msg_3));
        }
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "integration_testing"), ignore)]
    async fn test_list_attempts_by_endpoint() {
        let (client, _jh) = start_svix_server();

        let app_id = create_test_app(&client, "v1AttemptListAttemptsByEndpointTestApp")
            .await
            .unwrap();

        let receiver_1 = TestReceiver::start(axum::http::StatusCode::OK);
        let receiver_2 = TestReceiver::start(axum::http::StatusCode::OK);

        let endp_id_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
            .await
            .unwrap();
        let endp_id_2 = create_test_endpoint(&client, &app_id, &receiver_2.endpoint)
            .await
            .unwrap();

        let msg_1 = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
            .await
            .unwrap();
        let msg_2 = create_test_message(&client, &app_id, serde_json::json!({"test": "data2"}))
            .await
            .unwrap();
        let msg_3 = create_test_message(&client, &app_id, serde_json::json!({"test": "data3"}))
            .await
            .unwrap();

        // And wait at most one second for all attempts to be processed
        run_with_retries(|| async {
            for endp_id in [endp_id_1.clone(), endp_id_2.clone()] {
                let list: ListResponse<MessageAttemptOut> = client
                    .get(
                        &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, endp_id),
                        StatusCode::OK,
                    )
                    .await
                    .unwrap();

                if list.data.len() != 3 {
                    anyhow::bail!("list len {}, not 3", list.data.len());
                }
            }

            Ok(())
        })
        .await
        .unwrap();

        let list_1: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, endp_id_1),
                StatusCode::OK,
            )
            .await
            .unwrap();
        let list_2: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, endp_id_2),
                StatusCode::OK,
            )
            .await
            .unwrap();

        for list in [list_1, list_2] {
            let message_ids: Vec<_> = list.data.into_iter().map(|amo| amo.msg_id).collect();
            assert!(message_ids.contains(&msg_1));
            assert!(message_ids.contains(&msg_2));
            assert!(message_ids.contains(&msg_3));
        }

        receiver_1.jh.abort();
        receiver_2.jh.abort();
    }
}
