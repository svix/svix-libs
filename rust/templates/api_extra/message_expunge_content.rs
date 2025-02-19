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
