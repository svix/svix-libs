// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, BaseId, EndpointId, EndpointIdOrUid, EventChannel,
            MessageAttemptId, MessageAttemptTriggerType, MessageIdOrUid, MessageStatus,
        },
    },
    db::models::{endpoint, message, messagedestination},
    error::{HttpError, Result},
    queue::{MessageTask, TaskQueueProducer},
    v1::utils::{
        api_not_implemented, EmptyResponse, ListResponse, MessageListFetchOptions, ModelOut,
        ValidatedQuery,
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};

use hyper::StatusCode;
use sea_orm::{entity::prelude::*, QueryOrder};
use sea_orm::{DatabaseConnection, QuerySelect};
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
            endpoint_id: model.endp_id,

            id: model.id,
            created_at: model.created_at.into(),
        }
    }
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
    Path((_app_id, msg_id, endpoint_id)): Path<(
        ApplicationIdOrUid,
        MessageIdOrUid,
        EndpointIdOrUid,
    )>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
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
        "/app/:app_id/msg/:msg_id",
        Router::new()
            .route("/attempt/", get(list_messageattempts))
            .route("/attempt/:attempt_id/", get(get_messageattempt))
            .route("/endpoint/", get(api_not_implemented))
            .route("/endpoint/:endpoint_id/resend/", post(resend_webhook))
            .route("/endpoint/:endpoint_id/attempt/", get(api_not_implemented)),
        // FIXME: Missing the one for list attempted messages
    )
}
