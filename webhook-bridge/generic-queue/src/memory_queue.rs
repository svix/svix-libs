use std::{fmt::Debug, marker::PhantomData, time::Duration};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::broadcast;

use crate::{Delivery, QueueError, TaskQueueBackend, TaskQueueReceive, TaskQueueSend};

pub struct MemoryQueueBackend<
    T: 'static + Clone + Debug + DeserializeOwned + Send + Serialize + Sync,
>(PhantomData<T>);

#[async_trait]
impl<T: 'static + Clone + Debug + DeserializeOwned + Send + Serialize + Sync> TaskQueueBackend<T>
    for MemoryQueueBackend<T>
{
    type PairConfig = usize;

    type Delivery = MemoryQueueDelivery<T>;

    type Producer = MemoryQueueProducer<T>;
    type Consumer = MemoryQueueConsumer<T>;

    async fn new_pair(
        cfg: usize,
    ) -> Result<(MemoryQueueProducer<T>, MemoryQueueConsumer<T>), QueueError> {
        let (tx, rx) = broadcast::channel(cfg);

        let producer = MemoryQueueProducer { tx: tx.clone() };
        let consumer = MemoryQueueConsumer { tx, rx };

        Ok((producer, consumer))
    }

    async fn producing_half(_cfg: usize) -> Result<MemoryQueueProducer<T>, QueueError> {
        Err(QueueError::CannotCreateHalf)
    }

    async fn consuming_half(_cfg: usize) -> Result<MemoryQueueConsumer<T>, QueueError> {
        Err(QueueError::CannotCreateHalf)
    }
}

#[derive(Clone)]
pub struct MemoryQueueDelivery<T> {
    payload: T,
    ack_tx: broadcast::Sender<T>,
}

#[async_trait]
impl<T: 'static + Clone + Debug + DeserializeOwned + Send + Serialize + Sync> Delivery<T>
    for MemoryQueueDelivery<T>
{
    fn payload(&self) -> Result<T, QueueError> {
        Ok(self.payload.clone())
    }

    async fn ack(self) -> Result<(), QueueError> {
        Ok(())
    }

    async fn nack(self) -> Result<(), QueueError> {
        self.ack_tx
            .send(self.payload)
            .map_err(QueueError::generic)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct MemoryQueueProducer<T: Send + Serialize + Sync> {
    tx: broadcast::Sender<T>,
}

#[async_trait]
impl<T: 'static + Clone + Debug + Send + Serialize + Sync> TaskQueueSend<T>
    for MemoryQueueProducer<T>
{
    async fn send(&self, payload: T) -> Result<(), QueueError> {
        self.tx.send(payload).map_err(QueueError::generic)?;
        Ok(())
    }
}

pub struct MemoryQueueConsumer<T: DeserializeOwned + Serialize + Send + Sync> {
    tx: broadcast::Sender<T>,
    rx: broadcast::Receiver<T>,
}

#[async_trait]
impl<T: 'static + Clone + Debug + DeserializeOwned + Serialize + Send + Sync>
    TaskQueueReceive<T, MemoryQueueDelivery<T>> for MemoryQueueConsumer<T>
{
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<MemoryQueueDelivery<T>>, QueueError> {
        let mut out = Vec::with_capacity(max_batch_size);

        // Await at least one delivery before starting the clock
        out.push(
            self.rx
                .recv()
                .await
                .map(|payload| MemoryQueueDelivery {
                    payload,
                    ack_tx: self.tx.clone(),
                })
                .map_err(QueueError::generic)?,
        );

        let mut interval = tokio::time::interval(timeout);

        // Skip the first tick which is instantaneous
        interval.tick().await;

        loop {
            tokio::select! {
                _ = interval.tick() => break,
                msg = self.rx.recv() => {
                    out.push(
                        msg.map(|payload| MemoryQueueDelivery {
                            payload,
                            ack_tx: self.tx.clone(),
                        })
                        .map_err(QueueError::generic)?,
                    );

                    if out.len() >= max_batch_size {
                        break;
                    }
                }
            }
        }

        Ok(out)
    }
}

impl<T: DeserializeOwned + Serialize + Send + Sync> Clone for MemoryQueueConsumer<T> {
    fn clone(&self) -> Self {
        Self {
            rx: self.tx.subscribe(),
            tx: self.tx.clone(),
        }
    }
}
