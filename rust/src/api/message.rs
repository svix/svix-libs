use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct MessageListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filter response based on the channel.
    pub channel: Option<String>,

    /// Only include items created before a certain date.
    ///
    /// RFC3339 date string.
    pub before: Option<String>,

    /// Only include items created after a certain date.
    ///
    /// RFC3339 date string.
    pub after: Option<String>,

    /// When `true` message payloads are included in the response.
    pub with_content: Option<bool>,

    /// Filter messages matching the provided tag.
    pub tag: Option<String>,

    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
}

#[derive(Default)]
pub struct MessageCreateOptions {
    /// When `true`, message payloads are included in the response.
    pub with_content: Option<bool>,

    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct MessageExpungeAllContentsOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct MessageGetOptions {
    /// When `true` message payloads are included in the response.
    pub with_content: Option<bool>,
}

pub struct Message<'a> {
    cfg: &'a Configuration,
}

impl<'a> Message<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List all of the application's messages.
    ///
    /// The `before` and `after` parameters let you filter all items created
    /// before or after a certain date. These can be used alongside an iterator
    /// to paginate over results within a certain window.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days'
    /// worth of data relative to now or, if an iterator is provided, 90
    /// days before/after the time indicated by the iterator ID. If you
    /// require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    pub async fn list(
        &self,
        app_id: String,
        options: Option<MessageListOptions>,
    ) -> Result<ListResponseMessageOut> {
        let MessageListOptions {
            limit,
            iterator,
            channel,
            before,
            after,
            with_content,
            tag,
            event_types,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/msg")
            .with_path_param("app_id", app_id)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("channel", channel)
            .with_optional_query_param("before", before)
            .with_optional_query_param("after", after)
            .with_optional_query_param("with_content", with_content)
            .with_optional_query_param("tag", tag)
            .with_optional_query_param("event_types", event_types)
            .execute(self.cfg)
            .await
    }

    /// Creates a new message and dispatches it to all of the application's
    /// endpoints.
    ///
    /// The `eventId` is an optional custom unique ID. It's verified to be
    /// unique only up to a day, after that no verification will be made. If
    /// a message with the same `eventId` already exists for the application, a
    /// 409 conflict error will be returned.
    ///
    /// The `eventType` indicates the type and schema of the event. All messages
    /// of a certain `eventType` are expected to have the same schema. Endpoints
    /// can choose to only listen to specific event types. Messages can also
    /// have `channels`, which similar to event types let endpoints filter by
    /// them. Unlike event types, messages can have multiple channels, and
    /// channels don't imply a specific message content or schema.
    ///
    /// The `payload` property is the webhook's body (the actual webhook
    /// message). Svix supports payload sizes of up to ~350kb, though it's
    /// generally a good idea to keep webhook payloads small, probably no larger
    /// than 40kb.
    pub async fn create(
        &self,
        app_id: String,
        message_in: MessageIn,
        options: Option<MessageCreateOptions>,
    ) -> Result<MessageOut> {
        let MessageCreateOptions {
            with_content,
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/api/v1/app/{app_id}/msg")
            .with_path_param("app_id", app_id)
            .with_optional_query_param("with_content", with_content)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(message_in)
            .execute(self.cfg)
            .await
    }

    /// Purge all message content for the application.
    ///
    /// Delete all message payloads for the application.
    pub async fn expunge_all_contents(
        &self,
        app_id: String,
        options: Option<MessageExpungeAllContentsOptions>,
    ) -> Result<ExpungAllContentsOut> {
        let MessageExpungeAllContentsOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/msg/expunge-all-contents",
        )
        .with_path_param("app_id", app_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .execute(self.cfg)
        .await
    }

    /// Get a message by its ID or eventID.
    pub async fn get(
        &self,
        app_id: String,
        msg_id: String,
        options: Option<MessageGetOptions>,
    ) -> Result<MessageOut> {
        let MessageGetOptions { with_content } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/msg/{msg_id}")
            .with_path_param("app_id", app_id)
            .with_path_param("msg_id", msg_id)
            .with_optional_query_param("with_content", with_content)
            .execute(self.cfg)
            .await
    }

    /// Delete the given message's payload.
    ///
    /// Useful in cases when a message was accidentally sent with sensitive
    /// content. The message can't be replayed or resent once its payload
    /// has been deleted or expired.
    pub async fn expunge_content(&self, app_id: String, msg_id: String) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/app/{app_id}/msg/{msg_id}/content",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("msg_id", msg_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    #[cfg(feature = "svix_beta")]
    pub async fn events(
        &self,
        params: V1MessageEventsParams,
    ) -> Result<crate::models::MessageEventsOut> {
        let V1MessageEventsParams {
            app_id,
            limit,
            iterator,
            event_types,
            channels,
            after,
        } = params;

        crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/events")
            .with_path_param("app_id", app_id)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("event_types", event_types)
            .with_optional_query_param("channels", channels)
            .with_optional_query_param("after", after)
            .execute(self.cfg)
            .await
    }

    #[cfg(feature = "svix_beta")]
    pub async fn events_subscription(
        &self,
        params: V1MessageEventsSubscriptionParams,
    ) -> Result<crate::models::MessageEventsOut> {
        let V1MessageEventsSubscriptionParams {
            app_id,
            subscription_id,
            limit,
            iterator,
            event_types,
            channels,
            after,
        } = params;

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/events/subscription/{subscription_id}",
        )
        .with_path_param("app_id", app_id.to_string())
        .with_path_param("subscription_id", subscription_id.to_string())
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("event_types", event_types)
        .with_optional_query_param("channels", channels)
        .with_optional_query_param("after", after)
        .execute(self.cfg)
        .await
    }
}

#[cfg(feature = "svix_beta")]
#[derive(Clone, Debug)]
pub struct V1MessageEventsParams {
    /// The app's ID or UID
    pub app_id: String,
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
    /// Filter response based on the event type.
    pub channels: Option<Vec<String>>,
    pub after: Option<String>,
}

#[cfg(feature = "svix_beta")]
#[derive(Clone, Debug)]
pub struct V1MessageEventsSubscriptionParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The esub's ID or UID
    pub subscription_id: String,
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
    /// Filter response based on the event type.
    pub channels: Option<Vec<String>>,
    pub after: Option<String>,
}
