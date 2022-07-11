// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use axum::{extract::Extension, Router};

use cfg::CacheType;
use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::{
    cfg::Configuration,
    core::{
        cache,
        idempotency::IdempotencyService,
        operational_webhooks::OperationalWebhookSenderInner,
        otel_spans::{AxumOtelOnFailure, AxumOtelOnResponse, AxumOtelSpanCreator},
    },
    db::init_db,
    expired_message_cleaner::expired_message_cleaner_loop,
    worker::worker_loop,
};

pub mod cfg;
pub mod core;
pub mod db;
pub mod error;
pub mod expired_message_cleaner;
pub mod queue;
pub mod redis;
pub mod v1;
pub mod worker;

#[tracing::instrument(name = "app_start", level = "trace", skip_all)]
pub async fn run(cfg: Configuration, listener: Option<TcpListener>) {
    run_with_prefix(None, cfg, listener).await
}

// Made public for the purpose of E2E testing in which a queue prefix is necessary to avoid tests
// consuming from each others' queues
pub async fn run_with_prefix(
    prefix: Option<String>,
    cfg: Configuration,
    listener: Option<TcpListener>,
) {
    let pool = init_db(&cfg).await;

    let redis_dsn = || {
        cfg.redis_dsn
            .as_ref()
            .expect("Redis DSN not found")
            .as_str()
    };

    tracing::debug!("Cache type: {:?}", cfg.cache_type);
    let cache = match cfg.cache_type {
        CacheType::Redis => {
            let mgr = crate::redis::new_redis_pool(redis_dsn(), &cfg).await;
            cache::redis::new(mgr)
        }
        CacheType::RedisCluster => {
            let mgr = crate::redis::new_redis_pool_clustered(redis_dsn(), &cfg).await;
            cache::redis::new(mgr)
        }
        CacheType::Memory => cache::memory::new(),
        CacheType::None => cache::none::new(),
    };

    tracing::debug!("Queue type: {:?}", cfg.queue_type);
    let (queue_tx, queue_rx) = queue::new_pair(&cfg, prefix.as_deref()).await;

    let op_webhook_sender = OperationalWebhookSenderInner::new(
        cfg.jwt_secret.clone(),
        cfg.operational_webhook_address.clone(),
    );

    // build our application with a route
    let app = Router::new()
        .nest("/api/v1", v1::router())
        .merge(docs::router())
        .layer(
            ServiceBuilder::new().layer_fn(|service| IdempotencyService {
                cache: cache.clone(),
                service,
            }),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(AllowHeaders::mirror_request())
                .max_age(Duration::from_secs(600)),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(AxumOtelSpanCreator)
                .on_response(AxumOtelOnResponse)
                .on_failure(AxumOtelOnFailure),
        )
        .layer(Extension(pool.clone()))
        .layer(Extension(queue_tx.clone()))
        .layer(Extension(cfg.clone()))
        .layer(Extension(cache.clone()))
        .layer(Extension(op_webhook_sender.clone()));

    let with_api = cfg.api_enabled;
    let with_worker = cfg.worker_enabled;

    let listen_address =
        SocketAddr::from_str(&cfg.listen_address).expect("Error parsing server listen address");
    let (server, worker_loop, expired_message_cleaner_loop) = tokio::join!(
        async {
            if with_api {
                if let Some(l) = listener {
                    tracing::debug!("API: Listening on {}", l.local_addr().unwrap());
                    axum::Server::from_tcp(l)
                        .expect("Error starting http server")
                        .serve(app.into_make_service())
                        .await
                } else {
                    tracing::debug!("API: Listening on {}", listen_address);
                    axum::Server::bind(&listen_address)
                        .serve(app.into_make_service())
                        .await
                }
            } else {
                tracing::debug!("API: off");
                Ok(())
            }
        },
        async {
            if with_worker {
                tracing::debug!("Worker: Initializing");
                worker_loop(&cfg, &pool, cache, queue_tx, queue_rx, op_webhook_sender).await
            } else {
                tracing::debug!("Worker: off");
                Ok(())
            }
        },
        async {
            if with_worker {
                tracing::debug!("Expired message cleaner: Initializing");
                expired_message_cleaner_loop(&pool).await
            } else {
                tracing::debug!("Expired message cleaner: off");
                Ok(())
            }
        }
    );
    server.expect("Error initializing server");
    worker_loop.expect("Error initializing worker");
    expired_message_cleaner_loop.expect("Error initializing expired message cleaner")
}

mod docs {
    use axum::{
        response::{Html, IntoResponse, Redirect},
        routing::get,
        Json, Router,
    };

    pub fn router() -> Router {
        Router::new()
            .route("/", get(|| async { Redirect::temporary("/docs") }))
            .route("/docs", get(get_docs))
            .route("/api/v1/openapi.json", get(get_openapi_json))
    }

    async fn get_docs() -> Html<&'static str> {
        Html(include_str!("static/docs.html"))
    }

    async fn get_openapi_json() -> impl IntoResponse {
        let json: serde_json::Value = serde_json::from_str(include_str!("static/openapi.json"))
            .expect("Error: openapi.json does not exist");
        Json(json)
    }
}
