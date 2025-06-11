use std::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use aws_sdk_sqs::{
    operation::delete_message::DeleteMessageError, types::error::ReceiptHandleIsInvalid, Client,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Delivery, QueueError, TaskQueueBackend, TaskQueueReceive, TaskQueueSend};

pub struct SqsConfig {
    pub queue_dsn: String,
    pub override_endpoint: bool,
}

pub struct SqsQueueBackend;

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync> TaskQueueBackend<T>
    for SqsQueueBackend
{
    type PairConfig = SqsConfig;

    type Delivery = SqsDelivery<T>;

    type Producer = SqsQueueProducer;
    type Consumer = SqsQueueConsumer;

    async fn new_pair(cfg: SqsConfig) -> Result<(SqsQueueProducer, SqsQueueConsumer), QueueError> {
        let aws_cfg = if cfg.override_endpoint {
            aws_config::from_env()
                .endpoint_url(&cfg.queue_dsn)
                .load()
                .await
        } else {
            aws_config::load_from_env().await
        };

        let client = Client::new(&aws_cfg);

        let producer = SqsQueueProducer {
            client: client.clone(),
            queue_dsn: cfg.queue_dsn.clone(),
        };
        let consumer = SqsQueueConsumer {
            client,
            queue_dsn: cfg.queue_dsn,
        };

        Ok((producer, consumer))
    }

    async fn producing_half(cfg: SqsConfig) -> Result<SqsQueueProducer, QueueError> {
        let aws_cfg = if cfg.override_endpoint {
            aws_config::from_env()
                .endpoint_url(&cfg.queue_dsn)
                .load()
                .await
        } else {
            aws_config::load_from_env().await
        };

        let client = Client::new(&aws_cfg);

        let producer = SqsQueueProducer {
            client,
            queue_dsn: cfg.queue_dsn,
        };

        Ok(producer)
    }

    async fn consuming_half(cfg: SqsConfig) -> Result<SqsQueueConsumer, QueueError> {
        let aws_cfg = if cfg.override_endpoint {
            aws_config::from_env()
                .endpoint_url(&cfg.queue_dsn)
                .load()
                .await
        } else {
            aws_config::load_from_env().await
        };

        let client = Client::new(&aws_cfg);

        let consumer = SqsQueueConsumer {
            client,
            queue_dsn: cfg.queue_dsn,
        };

        Ok(consumer)
    }
}

pub struct SqsDelivery<T: DeserializeOwned> {
    ack_client: Client,
    // FIXME: Cow/Arc this stuff?
    queue_dsn: String,
    body: String,
    receipt_handle: Option<String>,
    _pd: PhantomData<T>,
}

#[async_trait]
impl<T: DeserializeOwned + Send + Serialize + Sync> Delivery<T> for SqsDelivery<T> {
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

pub struct SqsQueueProducer {
    client: Client,
    queue_dsn: String,
}

#[async_trait]
impl<T: 'static + Serialize + Send + Sync> TaskQueueSend<T> for SqsQueueProducer {
    async fn send(&self, payload: T) -> Result<(), QueueError> {
        self.client
            .send_message()
            .queue_url(&self.queue_dsn)
            .message_body(serde_json::to_string(&payload)?)
            .send()
            .await
            .map_err(QueueError::generic)?;

        Ok(())
    }
}

pub struct SqsQueueConsumer {
    client: Client,
    queue_dsn: String,
}

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync> TaskQueueReceive<T, SqsDelivery<T>>
    for SqsQueueConsumer
{
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<SqsDelivery<T>>, QueueError> {
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
            .map(|message| -> Result<SqsDelivery<T>, QueueError> {
                Ok(SqsDelivery {
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
