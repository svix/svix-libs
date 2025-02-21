use std::{collections::HashMap, marker::PhantomData, time::Duration};

use async_trait::async_trait;
use bb8_redis::RedisConnectionManager;
use redis::streams::{StreamId, StreamKey, StreamReadOptions, StreamReadReply};
use serde::{
    de::{DeserializeOwned, Error, Unexpected},
    Serialize,
};

use crate::{Delivery, QueueError, TaskQueueBackend, TaskQueueReceive, TaskQueueSend};

pub struct RedisConfig {
    pub dsn: String,
    pub max_connections: u16,
    pub reinsert_on_nack: bool,
    pub queue_key: String,
    pub consumer_group: String,
    pub consumer_name: String,
}

pub struct RedisQueueBackend<S: RedisStreamSerdeScheme = RedisStreamJsonSerde> {
    _pd_serde: PhantomData<S>,
}

#[async_trait]
impl<
        S: RedisStreamSerdeScheme,
        T: 'static + RedisStreamDeserialize<S> + RedisStreamSerialize<S> + Send + Sync,
    > TaskQueueBackend<T> for RedisQueueBackend<S>
{
    type PairConfig = RedisConfig;

    type Delivery = RedisStreamDelivery<T, S>;

    type Producer = RedisStreamProducer<S>;
    type Consumer = RedisStreamConsumer<S>;

    async fn new_pair(
        cfg: RedisConfig,
    ) -> Result<(RedisStreamProducer<S>, RedisStreamConsumer<S>), QueueError> {
        let redis = RedisConnectionManager::new(cfg.dsn).map_err(QueueError::generic)?;
        let redis = bb8::Pool::builder()
            .max_size(cfg.max_connections.into())
            .build(redis)
            .await
            .map_err(QueueError::generic)?;

        Ok((
            RedisStreamProducer {
                redis: redis.clone(),
                queue_key: cfg.queue_key.clone(),
                _pd_serde: PhantomData,
            },
            RedisStreamConsumer {
                redis,
                queue_key: cfg.queue_key,
                consumer_group: cfg.consumer_group,
                consumer_name: cfg.consumer_name,
                reinsert_on_nack: cfg.reinsert_on_nack,
                _pd_serde: PhantomData,
            },
        ))
    }

    async fn producing_half(cfg: RedisConfig) -> Result<RedisStreamProducer<S>, QueueError> {
        let redis = RedisConnectionManager::new(cfg.dsn).map_err(QueueError::generic)?;
        let redis = bb8::Pool::builder()
            .max_size(cfg.max_connections.into())
            .build(redis)
            .await
            .map_err(QueueError::generic)?;

        Ok(RedisStreamProducer {
            redis,
            queue_key: cfg.queue_key,
            _pd_serde: PhantomData,
        })
    }

    async fn consuming_half(cfg: RedisConfig) -> Result<RedisStreamConsumer<S>, QueueError> {
        let redis = RedisConnectionManager::new(cfg.dsn).map_err(QueueError::generic)?;
        let redis = bb8::Pool::builder()
            .max_size(cfg.max_connections.into())
            .build(redis)
            .await
            .map_err(QueueError::generic)?;

        Ok(RedisStreamConsumer {
            redis,
            queue_key: cfg.queue_key,
            consumer_group: cfg.consumer_group,
            consumer_name: cfg.consumer_name,
            reinsert_on_nack: cfg.reinsert_on_nack,
            _pd_serde: PhantomData,
        })
    }
}

pub trait RedisStreamSerdeScheme: Send + Sized + Sync {}
pub trait RedisStreamSerialize<S: RedisStreamSerdeScheme> {
    fn into_redis_stream_map(self) -> Result<Vec<(String, String)>, QueueError>;
}
pub trait RedisStreamDeserialize<S: RedisStreamSerdeScheme>: Sized {
    fn from_redis_stream_map(map: &HashMap<String, redis::Value>) -> Result<Self, QueueError>;
}

pub struct RedisStreamJsonSerde;
impl RedisStreamSerdeScheme for RedisStreamJsonSerde {}

impl<T: Serialize> RedisStreamSerialize<RedisStreamJsonSerde> for T {
    fn into_redis_stream_map(self) -> Result<Vec<(String, String)>, QueueError> {
        Ok(vec![("payload".to_owned(), serde_json::to_string(&self)?)])
    }
}

impl<T: DeserializeOwned> RedisStreamDeserialize<RedisStreamJsonSerde> for T {
    fn from_redis_stream_map(map: &HashMap<String, redis::Value>) -> Result<Self, QueueError> {
        let Some(payload) = map.get("payload") else {
            return Err(QueueError::Serde(serde_json::Error::missing_field(
                "payload",
            )))
        };

        match payload {
            redis::Value::Data(bytes) => {
                let s = std::str::from_utf8(bytes).map_err(|_| {
                    QueueError::Serde(serde_json::Error::invalid_type(
                        Unexpected::Other("unknown"),
                        &"string",
                    ))
                })?;

                Ok(serde_json::from_str(s)?)
            }
            redis::Value::Nil => Err(QueueError::Serde(serde_json::Error::invalid_type(
                Unexpected::Other("nil"),
                &"string",
            ))),
            redis::Value::Int(i) => Err(QueueError::Serde(serde_json::Error::invalid_type(
                Unexpected::Signed(*i),
                &"string",
            ))),
            redis::Value::Bulk(_) => Err(QueueError::Serde(serde_json::Error::invalid_type(
                Unexpected::Seq,
                &"string",
            ))),
            redis::Value::Status(_) => Err(QueueError::Serde(serde_json::Error::invalid_type(
                Unexpected::Other("status"),
                &"string",
            ))),
            redis::Value::Okay => Err(QueueError::Serde(serde_json::Error::invalid_type(
                Unexpected::Other("okay"),
                &"string",
            ))),
        }
    }
}

pub struct RedisStreamDelivery<T, U: RedisStreamSerdeScheme> {
    body: StreamId,

    reinsert_on_nack: bool,

    redis: bb8::Pool<RedisConnectionManager>,
    queue_key: String,
    consumer_group: String,
    entry_id: String,

    _pd_payload: PhantomData<T>,
    _pd_serde: PhantomData<U>,
}

#[async_trait]
impl<
        S: RedisStreamSerdeScheme,
        T: RedisStreamDeserialize<S> + RedisStreamSerialize<S> + Send + Sync,
    > Delivery<T> for RedisStreamDelivery<T, S>
{
    fn payload(&self) -> Result<T, QueueError> {
        T::from_redis_stream_map(&self.body.map).map_err(Into::into)
    }

    async fn ack(self) -> Result<(), QueueError> {
        let mut conn = self.redis.get().await.map_err(QueueError::generic)?;
        redis::Cmd::xack(self.queue_key, self.consumer_group, &[self.entry_id])
            .query_async(&mut *conn)
            .await
            .map_err(QueueError::generic)?;

        Ok(())
    }

    async fn nack(self) -> Result<(), QueueError> {
        if self.reinsert_on_nack {
            let mut conn = self.redis.get().await.map_err(QueueError::generic)?;

            // FIXME: Transaction?
            redis::Cmd::xadd(
                &self.queue_key,
                "*",
                &T::from_redis_stream_map(&self.body.map)?.into_redis_stream_map()?,
            )
            .query_async(&mut *conn)
            .await
            .map_err(QueueError::generic)?;

            redis::Cmd::xack(self.queue_key, self.consumer_group, &[self.entry_id])
                .query_async(&mut *conn)
                .await
                .map_err(QueueError::generic)?;

            Ok(())
        } else {
            Ok(())
        }
    }
}

pub struct RedisStreamProducer<S: RedisStreamSerdeScheme> {
    redis: bb8::Pool<RedisConnectionManager>,
    queue_key: String,

    _pd_serde: PhantomData<S>,
}

#[async_trait]
impl<S: RedisStreamSerdeScheme, T: 'static + RedisStreamSerialize<S> + Send + Sync> TaskQueueSend<T>
    for RedisStreamProducer<S>
{
    async fn send(&self, payload: T) -> Result<(), QueueError> {
        let mut conn = self.redis.get().await.map_err(QueueError::generic)?;
        redis::Cmd::xadd(&self.queue_key, "*", &payload.into_redis_stream_map()?)
            .query_async(&mut *conn)
            .await
            .map_err(QueueError::generic)?;

        Ok(())
    }
}

pub struct RedisStreamConsumer<U: RedisStreamSerdeScheme> {
    redis: bb8::Pool<RedisConnectionManager>,
    queue_key: String,
    consumer_group: String,
    consumer_name: String,
    reinsert_on_nack: bool,

    _pd_serde: PhantomData<U>,
}

#[async_trait]
impl<
        S: RedisStreamSerdeScheme,
        T: RedisStreamDeserialize<S> + RedisStreamSerialize<S> + Send + Sync,
    > TaskQueueReceive<T, RedisStreamDelivery<T, S>> for RedisStreamConsumer<S>
{
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<RedisStreamDelivery<T, S>>, QueueError> {
        let mut conn = self.redis.get().await.map_err(QueueError::generic)?;

        // Ensure an empty vec is never returned
        let queue: StreamKey = loop {
            let read_out: StreamReadReply = redis::Cmd::xread_options(
                &[&self.queue_key],
                &[">"],
                &StreamReadOptions::default()
                    .group(&self.consumer_group, &self.consumer_name)
                    .block(
                        timeout
                            .as_millis()
                            .try_into()
                            .map_err(QueueError::generic)?,
                    )
                    .count(max_batch_size),
            )
            .query_async(&mut *conn)
            .await
            .map_err(QueueError::generic)?;

            let queue = read_out.keys.into_iter().next().ok_or(QueueError::NoData)?;

            if !queue.ids.is_empty() {
                break queue;
            }
        };

        Ok(queue
            .ids
            .into_iter()
            .map(|stream_id| {
                Ok(RedisStreamDelivery {
                    entry_id: stream_id.id.clone(),
                    body: stream_id,
                    reinsert_on_nack: self.reinsert_on_nack,
                    redis: self.redis.clone(),
                    queue_key: self.queue_key.clone(),
                    consumer_group: self.consumer_group.clone(),
                    _pd_serde: PhantomData,
                    _pd_payload: PhantomData,
                })
            })
            .collect::<Result<Vec<_>, QueueError>>()?)
    }
}

// TODO: Test that the pending entries list doesn't fill
