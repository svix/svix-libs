#[deprecated = "Use `list_by_msg` instead, setting the `endpoint_id` in `options`."]
pub async fn list_attempts_for_endpoint(
    &self,
    app_id: String,
    msg_id: String,
    endpoint_id: String,
    options: Option<MessageAttemptListByMsgOptions>,
) -> Result<ListResponseMessageAttemptEndpointOut> {
    let MessageAttemptListByMsgOptions {
        iterator,
        limit,
        event_types,
        before,
        after,
        channel,
        tag,
        status,
        status_code_class: _,
        endpoint_id: _,
        with_content: _,
    } = options.unwrap_or_default();

    crate::request::Request::new(
        http1::Method::GET,
        "/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/attempt",
    )
    .with_optional_query_param("limit", limit)
    .with_optional_query_param("iterator", iterator)
    .with_optional_query_param("channel", channel)
    .with_optional_query_param("tag", tag)
    .with_optional_query_param("status", status)
    .with_optional_query_param("before", before)
    .with_optional_query_param("after", after)
    .with_optional_query_param("event_types", event_types)
    .with_path_param("app_id", app_id.to_string())
    .with_path_param("msg_id", msg_id.to_string())
    .with_path_param("endpoint_id", endpoint_id.to_string())
    .execute(self.cfg)
    .await
}
