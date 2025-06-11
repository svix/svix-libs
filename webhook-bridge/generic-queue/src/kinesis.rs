// Sharding -- how to represent this ?
// Docs indicate pulling records needs to visit shard after shard.
// Maybe we can hide this from the caller.

use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use aws_sdk_kinesis::primitives::Blob;
use aws_sdk_kinesis::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Delivery, QueueError, TaskQueueBackend, TaskQueueReceive, TaskQueueSend};

pub struct KinesisConfig {
    // Consuming
    consumer_arn: String,
    // Producing
    stream_name: String,
    partition_key: String,
}

pub struct KinesisQueueBackend;

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync> TaskQueueBackend<T>
    for KinesisQueueBackend
{
    type PairConfig = KinesisConfig;
    type Delivery = KinesisDelivery<T>;
    type Producer = KinesisQueueProducer;
    type Consumer = KinesisQueueConsumer;

    async fn new_pair(
        cfg: KinesisConfig,
    ) -> Result<(KinesisQueueProducer, KinesisQueueConsumer), QueueError> {
        let aws_cfg = aws_config::load_from_env().await;

        let client = Client::new(&aws_cfg);

        let producer = KinesisQueueProducer {
            client: client.clone(),
            partition_key: cfg.partition_key,
            stream_name: cfg.stream_name,
        };
        let consumer = KinesisQueueConsumer {
            client,
            consumer_arn: cfg.consumer_arn,
        };

        Ok((producer, consumer))
    }

    async fn producing_half(cfg: KinesisConfig) -> Result<KinesisQueueProducer, QueueError> {
        let aws_cfg = aws_config::load_from_env().await;

        let client = Client::new(&aws_cfg);

        let producer = KinesisQueueProducer {
            client,
            queue_dsn: cfg.queue_dsn,
        };

        Ok(producer)
    }

    async fn consuming_half(cfg: KinesisConfig) -> Result<KinesisQueueConsumer, QueueError> {
        let aws_cfg = if cfg.override_endpoint {
            aws_config::from_env()
                .endpoint_url(&cfg.queue_dsn)
                .load()
                .await
        } else {
            aws_config::load_from_env().await
        };

        let client = Client::new(&aws_cfg);

        let consumer = KinesisQueueConsumer {
            client,
            queue_dsn: cfg.queue_dsn,
        };

        Ok(consumer)
    }
}

pub struct KinesisDelivery<T: DeserializeOwned> {
    ack_client: Client,
    // FIXME: Cow/Arc this stuff?
    queue_dsn: String,
    body: String,
    receipt_handle: Option<String>,
    _pd: PhantomData<T>,
}

#[async_trait]
impl<T: DeserializeOwned + Send + Serialize + Sync> Delivery<T> for KinesisDelivery<T> {
    fn payload(&self) -> Result<T, QueueError> {
        serde_json::from_str(&self.body).map_err(Into::into)
    }

    async fn ack(self) -> Result<(), QueueError> {
        if let Some(receipt_handle) = self.receipt_handle {
            self.ack_client
                .delete_message()
                .queue_url(&self.queue_dsn)
                .receipt_handle(receipt_handle)
                .send()
                .await
                .map_err(QueueError::generic)?;

            Ok(())
        } else {
            Err(QueueError::generic(
                DeleteMessageError::ReceiptHandleIsInvalid(
                    ReceiptHandleIsInvalid::builder()
                        .message("receipt handle must be Some to be acked")
                        .build(),
                ),
            ))
        }
    }

    async fn nack(self) -> Result<(), QueueError> {
        Ok(())
    }
}

pub struct KinesisQueueProducer {
    client: Client,
    partition_key: String,
    stream_name: String,
}

#[async_trait]
impl<T: 'static + Serialize + Send + Sync> TaskQueueSend<T> for KinesisQueueProducer {
    async fn send(&self, payload: T) -> Result<(), QueueError> {
        let data = Blob::new(serde_json::to_string(&payload)?);
        self.client
            .put_record()
            .data(data)
            .partition_key(&self.partition_key)
            .stream_name(&self.stream_name)
            .send()
            .await
            .map_err(QueueError::generic)?;

        Ok(())
    }
}

pub struct KinesisQueueConsumer {
    client: Client,
    consumer_arn: String,
}

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync>
    TaskQueueReceive<T, KinesisDelivery<T>> for KinesisQueueConsumer
{
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<KinesisDelivery<T>>, QueueError> {
        // Ensure that there's at least one message before returning regardless of timeout
        let out = loop {
            let out = self
                .client
                .receive_message()
                .set_wait_time_seconds(Some(
                    timeout.as_secs().try_into().map_err(QueueError::generic)?,
                ))
                .set_max_number_of_messages(Some(
                    max_batch_size.try_into().map_err(QueueError::generic)?,
                ))
                .queue_url(&self.queue_dsn)
                .send()
                .await
                .map_err(QueueError::generic)?;

            if !out.messages().unwrap_or_default().is_empty() {
                break out;
            }
        };

        Ok(out
            .messages()
            .unwrap_or_default()
            .iter()
            .map(|message| -> Result<KinesisDelivery<T>, QueueError> {
                Ok(KinesisDelivery {
                    ack_client: self.client.clone(),
                    queue_dsn: self.queue_dsn.clone(),
                    body: message.body().unwrap_or_default().to_owned(),
                    receipt_handle: message.receipt_handle().map(ToOwned::to_owned),
                    _pd: PhantomData,
                })
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
}
