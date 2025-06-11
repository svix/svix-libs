use crate::config::{GCPPubSubOutputOpts, RabbitMqOutputOpts, RedisOutputOpts, SqsOutputOpts};
use crate::types::{SerializablePayload, SerializableRequest, Validated};
use anyhow::Result;
use axum::async_trait;
use enum_dispatch::enum_dispatch;
use generic_queue::gcp_pubsub::{GCPPubSubConfig, GCPPubSubQueueBackend};
use generic_queue::rabbitmq::{RabbitMqBackend, RabbitMqConfig};
use generic_queue::redis::{RedisConfig, RedisQueueBackend};
use generic_queue::sqs::{SqsConfig, SqsQueueBackend};
use generic_queue::{TaskQueueBackend, TaskQueueSend};
use std::sync::Arc;

#[async_trait]
#[enum_dispatch]
pub trait ForwardingMethod {
    async fn forward(&self, req: SerializableRequest<Validated>) -> Result<http::StatusCode>;
}

#[derive(Clone)]
pub struct GenericQueueForwarder {
    // FIXME: if we retain things like the queue name we can show this in the Debug impl
    sender: Arc<Box<dyn TaskQueueSend<serde_json::Value>>>,
}

type Msg = serde_json::Value;

impl GenericQueueForwarder {
    pub async fn from_rabbitmq_cfg(cfg: RabbitMqOutputOpts) -> Result<Self> {
        let sender = <RabbitMqBackend as TaskQueueBackend<Msg>>::producing_half(RabbitMqConfig {
            uri: cfg.uri,
            // N.b the connection properties type is not serde-friendly. If we want to expose some
            // of these settings we'll probably need to provide our own type and build the real one
            // here from cfg.
            connection_properties: Default::default(),
            publish_exchange: cfg.exchange,
            publish_routing_key: cfg.routing_key,
            publish_options: cfg.publish_options,
            publish_properites: cfg.publish_properties,
            // consumer stuff we don't care about
            consume_queue: "".to_string(),
            consumer_tag: "".to_string(),
            consume_options: Default::default(),
            consume_arguments: Default::default(),
            requeue_on_nack: false,
        })
        .await?;

        Ok(Self {
            sender: Arc::new(Box::new(sender)),
        })
    }

    pub async fn from_redis_cfg(cfg: RedisOutputOpts) -> Result<Self> {
        let sender = <RedisQueueBackend as TaskQueueBackend<Msg>>::producing_half(RedisConfig {
            dsn: cfg.dsn,
            max_connections: cfg.max_connections,
            queue_key: cfg.queue_key,
            // consumer stuff we don't really care about
            reinsert_on_nack: false,
            consumer_group: "".to_string(),
            consumer_name: "".to_string(),
        })
        .await?;

        Ok(Self {
            sender: Arc::new(Box::new(sender)),
        })
    }

    pub async fn from_sqs_cfg(cfg: SqsOutputOpts) -> Result<Self> {
        let sender = <SqsQueueBackend as TaskQueueBackend<Msg>>::producing_half(SqsConfig {
            queue_dsn: cfg.queue_dsn,
            override_endpoint: cfg.override_endpoint,
        })
        .await?;

        Ok(Self {
            sender: Arc::new(Box::new(sender)),
        })
    }

    pub async fn from_gcp_pupsub_cfg(cfg: GCPPubSubOutputOpts) -> Result<Self> {
        let sender =
            <GCPPubSubQueueBackend as TaskQueueBackend<Msg>>::producing_half(GCPPubSubConfig {
                topic: cfg.topic,
                credentials_file: cfg.credentials_file,
                // Don't need this. Subscriptions are for consumers only.
                subscription_id: String::new(),
            })
            .await?;

        Ok(Self {
            sender: Arc::new(Box::new(sender)),
        })
    }
}
impl std::fmt::Debug for GenericQueueForwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericQueueForwarder").finish()
    }
}
#[async_trait]
impl ForwardingMethod for GenericQueueForwarder {
    async fn forward(&self, req: SerializableRequest<Validated>) -> Result<http::StatusCode> {
        let payload = match req.payload() {
            SerializablePayload::Standard(data) => serde_json::from_slice(data)?,
            SerializablePayload::StringSerializable(s) => serde_json::from_str(s)?,
        };

        self.sender.send(payload).await?;
        Ok(http::StatusCode::OK)
    }
}

// FIXME: HTTP Forwarder

#[enum_dispatch(ForwardingMethod)]
#[derive(Clone, Debug)]
pub enum Forwarder {
    GenericQueueForwarder,
}
