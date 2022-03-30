// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, time::Duration};

use crate::{
    core::{
        cache::RedisCache,
        message_app::CreateMessageApp,
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, BaseId, EventChannel, EventChannelSet, EventTypeName,
            EventTypeNameSet, MessageAttemptTriggerType, MessageId, MessageIdOrUid, MessageStatus,
            MessageUid,
        },
    },
    db::models::messagedestination,
    error::{Error, HttpError, Result},
    queue::{MessageTask, TaskQueueProducer},
    v1::utils::{
        ListResponse, MessageListFetchOptions, ModelIn, ModelOut, ValidatedJson, ValidatedQuery,
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
use sea_orm::{sea_query::Expr, ActiveValue::Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect, TransactionTrait};
use serde::{Deserialize, Serialize};

use svix_server_derive::{ModelIn, ModelOut};
use validator::{Validate, ValidationError};

use crate::db::models::message;
use crate::v1::utils::Pagination;

pub fn validate_channels_msg(
    channels: &EventChannelSet,
) -> std::result::Result<(), ValidationError> {
    let len = channels.0.len();
    if !(1..=5).contains(&len) {
        Err(ValidationError::new(
            "Channels must have at least 1 and at most 5 items, or be set to null.",
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
pub struct MessageIn {
    #[validate]
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub uid: Option<MessageUid>,
    #[validate]
    pub event_type: EventTypeName,
    pub payload: serde_json::Value,

    #[validate(custom = "validate_channels_msg")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<EventChannelSet>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for MessageIn {
    type ActiveModel = message::ActiveModel;

    fn update_model(self, model: &mut message::ActiveModel) {
        model.uid = Set(self.uid);
        model.payload = Set(self.payload);
        model.event_type = Set(self.event_type);

        model.channels = Set(self.channels);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
pub struct MessageOut {
    #[serde(rename = "eventId")]
    pub uid: Option<MessageUid>,
    pub event_type: EventTypeName,
    pub payload: serde_json::Value,

    pub channels: Option<EventChannelSet>,

    pub id: MessageId,
    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
}

// FIXME: This can and should be a derive macro
impl From<message::Model> for MessageOut {
    fn from(model: message::Model) -> Self {
        Self {
            uid: model.uid,
            event_type: model.event_type,
            payload: model.payload,

            channels: model.channels,

            id: model.id,
            created_at: model.created_at.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct ListMessagesQueryParams {
    #[validate]
    channel: Option<EventChannel>,
}

async fn list_messages(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<MessageId>>,
    ValidatedQuery(ListMessagesQueryParams { channel }): ValidatedQuery<ListMessagesQueryParams>,
    list_filter: MessageListFetchOptions,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageOut>>> {
    let limit = pagination.limit;
    let iterator = pagination
        .iterator
        .clone()
        .or_else(|| list_filter.before.map(MessageId::start_id));

    let mut query = message::Entity::secure_find(app.id)
        .order_by_desc(message::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(message::Column::Id.lt(iterator));
    }

    if let Some(EventTypeNameSet(event_types)) = list_filter.event_types {
        query = query.filter(message::Column::EventType.is_in(event_types));
    }

    if let Some(channel) = channel {
        query = query.filter(Expr::cust_with_values("channels ?? ?", vec![channel]));
    }

    Ok(Json(MessageOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
        false,
    )))
}

async fn create_message(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Extension(redis_cache): Extension<Option<RedisCache>>,
    ValidatedJson(data): ValidatedJson<MessageIn>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<MessageOut>)> {
    let create_message_app = CreateMessageApp::layered_fetch(
        redis_cache.as_ref(),
        db,
        Some(app.clone()),
        app.id.clone(),
        app.org_id,
        Duration::from_secs(30),
    )
    .await?
    // Should never happen since you're giving it an existing Application, but just in case
    .ok_or_else(|| Error::Generic(format!("Application doesn't exist: {}", app.id)))?;

    let txn = db.begin().await?;
    let db = &txn;

    let msg = message::ActiveModel {
        app_id: Set(app.id.clone()),
        org_id: Set(permissions.org_id),
        ..data.into()
    };
    let msg = msg.insert(db).await?;

    let trigger_type = MessageAttemptTriggerType::Scheduled; // Just laying the groundwork for when we support passing it
    let empty_channel_set = HashSet::new();
    let mut msg_dests = vec![];
    let mut tasks = vec![];
    for endp in create_message_app.endpoints
        .into_iter()
        .filter(|endp| {
            // No disabled or deleted enpoints ever
          	!endp.disabled && !endp.deleted &&
            (
                // Manual attempt types go throguh regardless
                trigger_type == MessageAttemptTriggerType::Manual
                || (
                        // If an endpoint has event types and it matches ours, or has no event types
                        endp
                        .event_types_ids
                        .as_ref()
                        .map(|x| x.0.contains(&msg.event_type))
                        .unwrap_or(true)
                    &&
                        // If an endpoint has no channels accept all messages, otherwise only if their channels overlap.
                        // A message with no channels doesn't match an endpoint with channels.
                        endp
                        .channels
                        .as_ref()
                        .map(|x| !x.0.is_disjoint(msg.channels.as_ref().map(|x| &x.0).unwrap_or(&empty_channel_set)))
                        .unwrap_or(true)
                ))
        })
    {
        let msg_dest = messagedestination::ActiveModel {
            msg_id: Set(msg.id.clone()),
            endp_id: Set(endp.id.clone()),
            next_attempt: Set(Some(Utc::now().into())),
            status: Set(MessageStatus::Sending),
            ..Default::default()
        };
        msg_dests.push(msg_dest);
        tasks.push(
            MessageTask::new_task(
                msg.id.clone(),
                app.id.clone(),
                endp.id, MessageAttemptTriggerType::Scheduled));
    }
    if !msg_dests.is_empty() {
        messagedestination::Entity::insert_many(msg_dests)
            .exec(db)
            .await?;
    }
    txn.commit().await.unwrap();
    for task in tasks {
        queue_tx.send(task, None).await?;
    }

    Ok((StatusCode::ACCEPTED, Json(msg.into())))
}

async fn get_message(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, msg_id)): Path<(ApplicationIdOrUid, MessageIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<MessageOut>> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(msg.into()))
}

pub fn router() -> Router {
    Router::new().nest(
        "/app/:app_id",
        Router::new()
            .route("/msg/", get(list_messages))
            .route("/msg/", post(create_message))
            .route("/msg/:msg_id/", get(get_message)),
    )
}
