use crate::types::{SerializableHeaderMap, SerializablePayload, SerializableRequest, Unvalidated};
use anyhow::Result;
use axum::async_trait;
use enum_dispatch::enum_dispatch;
use std::sync::Arc;
use svix::webhooks::Webhook;

#[async_trait]
#[enum_dispatch]
pub trait VerificationMethod {
    async fn validate(&self, req: SerializableRequest<Unvalidated>) -> Result<bool>;

    fn want_string_rep(&self) -> bool {
        false
    }
    fn need_string_rep(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct SvixVerifier {
    webhook: Arc<Webhook>,
}

impl SvixVerifier {
    pub fn new(webhook: Arc<Webhook>) -> Self {
        Self { webhook }
    }
}

impl std::fmt::Debug for SvixVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SvixVerifier").finish()
    }
}

#[async_trait]
impl VerificationMethod for SvixVerifier {
    /// This [`VerificationMethod::validate`] implementation *requires* that the headers *and* payload
    /// be in their byte representations due to the requirements of the [`svix`] library. Please lazily
    /// convert these values such as to avoid pointless back-and-forth conversions.
    async fn validate(&self, req: SerializableRequest<Unvalidated>) -> Result<bool> {
        let headers = req.headers();
        let payload = req.payload();

        match (headers, payload) {
            (SerializableHeaderMap::Standard(headers), SerializablePayload::Standard(payload)) => {
                if self.webhook.verify(payload, headers).is_ok() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            _ => {
                anyhow::bail!("`SvixVerifier::validate` given string representations")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoVerifier;

#[async_trait]
impl VerificationMethod for NoVerifier {
    async fn validate(&self, _req: SerializableRequest<Unvalidated>) -> Result<bool> {
        Ok(true)
    }
}

// Allowed due to restrictions by [`enum_dispatch`] on variant names matching the structure names
#[allow(clippy::enum_variant_names)]
#[enum_dispatch(VerificationMethod)]
#[derive(Clone, Debug)]
pub enum Verifier {
    SvixVerifier,
    NoVerifier,
}

#[cfg(test)]
mod tests {
    use super::{SvixVerifier, VerificationMethod};
    use crate::types::SerializableRequest;
    use axum::extract::FromRequest;
    use std::sync::Arc;
    use svix::webhooks::Webhook;

    #[tokio::test]
    async fn test_svix_verification() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let webhook = Arc::new(Webhook::new(&secret).unwrap());

        let payload = "example payload".as_bytes();
        let timestamp = chrono::Utc::now().timestamp();
        let signature = webhook.sign("msg_valid", timestamp, payload).unwrap();

        let sv = SvixVerifier { webhook };

        let req = http::request::Request::builder()
            .method("POST")
            .uri("test.uri")
            .header("svix-id", "msg_valid")
            .header("svix-signature", signature.clone())
            .header("svix-timestamp", &format!("{timestamp}"))
            .body(axum::body::Full::new(payload))
            .unwrap();

        let sr = SerializableRequest::from_request(req, &()).await.unwrap();
        assert!(sv.validate(sr).await.unwrap());

        let req = http::request::Request::builder()
            .method("POST")
            .uri("test.uri")
            .header("svix-id", "msg_invalid")
            .header("svix-signature", signature)
            .header("svix-timestamp", &format!("{timestamp}"))
            .body(axum::body::Full::new(payload))
            .unwrap();

        let sr = SerializableRequest::from_request(req, &()).await.unwrap();
        assert!(!sv.validate(sr).await.unwrap());
    }
}
