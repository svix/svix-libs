use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use futures::StreamExt;
use lapin::{acker::Acker, Channel, Connection, Consumer};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Delivery, QueueError, TaskQueueBackend, TaskQueueReceive, TaskQueueSend};

pub use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions},
    types::FieldTable,
    BasicProperties, ConnectionProperties,
};

pub struct RabbitMqConfig {
    pub uri: String,
    pub connection_properties: ConnectionProperties,

    pub publish_exchange: String,
    pub publish_routing_key: String,
    pub publish_options: BasicPublishOptions,
    pub publish_properites: BasicProperties,

    pub consume_queue: String,
    pub consumer_tag: String,
    pub consume_options: BasicConsumeOptions,
    pub consume_arguments: FieldTable,

    pub requeue_on_nack: bool,
}

pub struct RabbitMqBackend;

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync> TaskQueueBackend<T>
    for RabbitMqBackend
{
    type PairConfig = RabbitMqConfig;

    type Delivery = RabbitMqDelivery<T>;

    type Producer = RabbitMqProducer;
    type Consumer = RabbitMqConsumer;

    async fn new_pair(
        cfg: RabbitMqConfig,
    ) -> Result<(RabbitMqProducer, RabbitMqConsumer), QueueError> {
        let conn = Connection::connect(&cfg.uri, cfg.connection_properties)
            .await
            .map_err(QueueError::generic)?;

        let channel_tx = conn.create_channel().await.map_err(QueueError::generic)?;
        let channel_rx = conn.create_channel().await.map_err(QueueError::generic)?;

        Ok((
            RabbitMqProducer {
                channel: channel_tx,
                exchange: cfg.publish_exchange,
                routing_key: cfg.publish_routing_key,
                options: cfg.publish_options,
                properties: cfg.publish_properites,
            },
            RabbitMqConsumer {
                consumer: channel_rx
                    .basic_consume(
                        &cfg.consume_queue,
                        &cfg.consumer_tag,
                        cfg.consume_options,
                        cfg.consume_arguments,
                    )
                    .await
                    .map_err(QueueError::generic)?,
                requeue_on_nack: cfg.requeue_on_nack,
            },
        ))
    }

    async fn producing_half(cfg: RabbitMqConfig) -> Result<RabbitMqProducer, QueueError> {
        let conn = Connection::connect(&cfg.uri, cfg.connection_properties)
            .await
            .map_err(QueueError::generic)?;

        let channel_tx = conn.create_channel().await.map_err(QueueError::generic)?;

        Ok(RabbitMqProducer {
            channel: channel_tx,
            exchange: cfg.publish_exchange,
            routing_key: cfg.publish_routing_key,
            options: cfg.publish_options,
            properties: cfg.publish_properites,
        })
    }

    async fn consuming_half(cfg: RabbitMqConfig) -> Result<RabbitMqConsumer, QueueError> {
        let conn = Connection::connect(&cfg.uri, cfg.connection_properties)
            .await
            .map_err(QueueError::generic)?;

        let channel_rx = conn.create_channel().await.map_err(QueueError::generic)?;

        Ok(RabbitMqConsumer {
            consumer: channel_rx
                .basic_consume(
                    &cfg.consume_queue,
                    &cfg.consumer_tag,
                    cfg.consume_options,
                    cfg.consume_arguments,
                )
                .await
                .map_err(QueueError::generic)?,
            requeue_on_nack: cfg.requeue_on_nack,
        })
    }
}

pub struct RabbitMqDelivery<T: DeserializeOwned> {
    requeue_on_nack: bool,
    acker: Acker,
    body: Vec<u8>,

    _pd: PhantomData<T>,
}

#[async_trait]
impl<T: DeserializeOwned + Send + Serialize + Sync> Delivery<T> for RabbitMqDelivery<T> {
    fn payload(&self) -> Result<T, QueueError> {
        serde_json::from_slice(&self.body).map_err(Into::into)
    }

    async fn ack(self) -> Result<(), QueueError> {
        self.acker
            .ack(BasicAckOptions { multiple: false })
            .await
            .map_err(QueueError::generic)
    }

    async fn nack(self) -> Result<(), QueueError> {
        self.acker
            .nack(BasicNackOptions {
                multiple: false,
                requeue: self.requeue_on_nack,
            })
            .await
            .map_err(QueueError::generic)
    }
}

pub struct RabbitMqProducer {
    channel: Channel,
    exchange: String,
    routing_key: String,
    options: BasicPublishOptions,
    properties: BasicProperties,
}

#[async_trait]
impl<T: 'static + Send + Serialize + Sync> TaskQueueSend<T> for RabbitMqProducer {
    async fn send(&self, payload: T) -> Result<(), QueueError> {
        self.channel
            .basic_publish(
                &self.exchange,
                &self.routing_key,
                self.options,
                &serde_json::to_vec(&payload)?,
                self.properties.clone(),
            )
            .await
            .map_err(QueueError::generic)?;

        Ok(())
    }
}

pub struct RabbitMqConsumer {
    consumer: Consumer,
    requeue_on_nack: bool,
}

#[async_trait]
impl<T: DeserializeOwned + Send + Serialize + Sync> TaskQueueReceive<T, RabbitMqDelivery<T>>
    for RabbitMqConsumer
{
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<RabbitMqDelivery<T>>, QueueError> {
        let mut stream = self
            .consumer
            .clone()
            .map(|l: Result<lapin::message::Delivery, lapin::Error>| -> Result<RabbitMqDelivery<T>, QueueError> {
                let l = l.map_err(QueueError::generic)?;
                Ok(RabbitMqDelivery {
                    acker: l.acker,
                    body: l.data,
                    requeue_on_nack: self.requeue_on_nack,
                    _pd: PhantomData,
                })
            });
        let mut out = Vec::new();

        if let Some(delivery) = stream.next().await {
            out.push(delivery?);

            let mut interval = tokio::time::interval(timeout);
            // Skip the instant first period
            interval.tick().await;

            loop {
                tokio::select! {
                    _ = interval.tick() => break,
                    delivery = stream.next() => {
                        if let Some(delivery) = delivery {
                            out.push(delivery?);

                            if out.len() >= max_batch_size {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(out)
    }
}
