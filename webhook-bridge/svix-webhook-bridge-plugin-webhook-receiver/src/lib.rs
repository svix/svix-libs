use crate::config::IntegrationConfig;
use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use svix_webhook_bridge_types::{async_trait, Plugin};
use tracing::instrument;
use types::{IntegrationId, IntegrationState, InternalState, SerializableRequest, Unvalidated};

pub mod config;
mod forwarding;
mod types;
mod verification;

pub const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const PLUGIN_VERS: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WebhookReceiverPluginConfig {
    pub listen_addr: SocketAddr,
    pub routes: Vec<IntegrationConfig>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WebhookReceiverPlugin {
    cfg: WebhookReceiverPluginConfig,
}

impl WebhookReceiverPlugin {
    pub fn new(cfg: WebhookReceiverPluginConfig) -> Self {
        Self { cfg }
    }
}

impl TryInto<Box<dyn Plugin>> for WebhookReceiverPluginConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(WebhookReceiverPlugin::new(self)))
    }
}

#[async_trait]
impl Plugin for WebhookReceiverPlugin {
    async fn run(&self) -> std::io::Result<()> {
        let addr = &self.cfg.listen_addr;
        let state = InternalState::from_routes(self.cfg.routes.as_slice())
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let router = Router::new()
            .route(
                "/webhook/:integration_id",
                post(route).put(route).get(route).patch(route),
            )
            .route(
                "/webhook/:integration_id/",
                post(route).put(route).get(route).patch(route),
            )
            .with_state(state);

        tracing::info!("Listening on: {addr}");
        axum::Server::bind(addr)
            .serve(router.into_make_service())
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

#[instrument(
    skip_all,
    level="error",
    fields(
        integration_id=integration_id.as_ref(),
        svixagent_plugin.name = PLUGIN_NAME,
        svixagent_plugin.vers = PLUGIN_VERS,
    )
)]
async fn route(
    Path(integration_id): Path<IntegrationId>,
    State(InternalState(id_map)): State<InternalState>,
    req: SerializableRequest<Unvalidated>,
) -> http::StatusCode {
    if let Some(IntegrationState {
        verifier,
        forwarder,
    }) = id_map.get(&integration_id)
    {
        match req.validate(verifier).await {
            Ok(req) => {
                tracing::debug!("forwarding request");
                req.forward(forwarder).await
            }
            Err(code) => {
                tracing::warn!("validation failed: {code}");
                code
            }
        }
    } else {
        tracing::trace!("integration not found");
        http::StatusCode::NOT_FOUND
    }
}
