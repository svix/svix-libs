/// Create the application with the given ID, or create a new one if it
/// doesn't exist yet.
pub async fn get_or_create(
    &self,
    application_in: ApplicationIn,
    options: Option<ApplicationCreateOptions>,
) -> Result<ApplicationOut> {
    let ApplicationCreateOptions { idempotency_key } = options.unwrap_or_default();

    crate::request::Request::new(http1::Method::POST, "/api/v1/app")
        .with_query_param("get_if_exists", "true".to_owned())
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(application_in)
        .execute(self.cfg)
        .await
}
