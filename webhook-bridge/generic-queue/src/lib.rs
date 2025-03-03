//! This crate is meant to act as an abstraction layer over many concrete queue implementations such
//! as to allow supporting multiple backends from your library or application.

use std::time::Duration;

use async_trait::async_trait;
use thiserror::Error;

#[cfg(feature = "gcp_pubsub")]
pub mod gcp_pubsub;
#[cfg(feature = "kinesis")]
pub mod kinesis;
#[cfg(feature = "memory_queue")]
pub mod memory_queue;
#[cfg(feature = "rabbitmq")]
pub mod rabbitmq;
#[cfg(feature = "redis")]
pub mod redis;
#[cfg(feature = "sqs")]
pub mod sqs;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("no data was returned in the specified timeframe")]
    NoData,
    #[error("cannot create this type without matching half")]
    CannotCreateHalf,
    #[error("error serializing or deserializing type: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("unknown error: {0}")]
    Generic(Box<dyn std::error::Error + Send + Sync>),
}

impl QueueError {
    fn generic<E: 'static + std::error::Error + Send + Sync>(e: E) -> Self {
        Self::Generic(Box::new(e))
    }
}

/// This trait is simply used to define basic types associated with a queue backend, as well as to
/// allow the simple creation of a [`TaskQueueBackend::new_pair`] from the associated configuration
/// type.
///
/// This trait is generic over the type sent and/or received through the queue, but it tends to be
/// defined with blanket implementations allowing you to send and receive multiple types through a
/// single [`Self::Producer`], [`Self::Consumer`] pair. The notable exception to this is the in-
/// memory queue which is implemented via [`tokio::broadcast`] channels, which accept only items of
/// the preselected type.
#[async_trait]
pub trait TaskQueueBackend<T: Send + Sync> {
    /// All necessary configuration for creating a sending/receiving pair should be contained in this
    /// configuration type.
    type PairConfig;

    /// Because the actual type delivered by a queue tends to include a lot of metadata, and becaue
    /// connection handles often have to be cloned into a delivery type to allow ACKing and NACKing,
    /// this associated type defines what is actually output by the [`Self::Consumer`].
    ///
    /// This type must implement [`Delivery`] which contains a method for accessing the inner body and
    /// relevant metadata.
    type Delivery: Delivery<T> + Send + Sync;

    /// This type is what actually allows sending messages down the queue.
    ///
    /// This type must implement [`TaskQueueSend`] which contains a method for dispatching an event
    /// down the queue -- [`TaskQueueSend::send`].
    type Producer: TaskQueueSend<T>;

    /// This type is what actually allows receiving messages from the queue.
    ///
    /// This type must implement [`TaskQueueReceive`] which contains a method for receiving a batch
    /// of events with a maximum batch size and timeout -- [`TaskQueueReceive::receive_all`].
    type Consumer: TaskQueueReceive<T, Self::Delivery>;

    /// Creates a new [`Self::Producer`], [`Self::Consumer`] pair of the types associated with the
    /// implementor of this trait.
    ///
    /// This takes a configuration type as a parameter ([`Self::PairConfig`]) which is unique depending
    /// on the implementor of this trait.
    async fn new_pair(
        cfg: Self::PairConfig,
    ) -> Result<(Self::Producer, Self::Consumer), QueueError>;

    /// Creates just the [`Self::Producer`] compared to [`Self::new_pair`].
    async fn producing_half(cfg: Self::PairConfig) -> Result<Self::Producer, QueueError>;

    /// Creates just the [`Self::Consumer`] comapared to [`Self::new_pair`].
    async fn consuming_half(cfg: Self::PairConfig) -> Result<Self::Consumer, QueueError>;
}

/// This trait is used to interface with deliveries for a specific queue. Implementors of this trait
/// will often contain, in addition to the payload and metadata, members necessary for ACKing or
/// NACKing a delivery.
///
/// As ACKing and NACKing is done from the delivery (such as to allow [`Send`]ing deliveries across
/// threads without also needing a handle to the consumer that produced this delivery), any
/// implementations of this trait should *avoid* implementing [`Clone`]. This will ensure that a
/// single delivery is not both ACKed and NACKed.
#[async_trait]
pub trait Delivery<T: Send + Sync> {
    /// Returns a freshly deserialized instance of the contained payload.
    fn payload(&self) -> Result<T, QueueError>;

    /// ACKs this message, which, depending on what backend you are using, may be a NOOP, or it may
    /// explicity acknowledge the successful processing the message.
    ///
    /// When ACKed, consumers will not see this exact message again.
    async fn ack(self) -> Result<(), QueueError>;
    /// NACKs this message, which, depending on what backend you are using, may be a NOOP, it may
    /// explicitly mark a messaege as not acknowledged, or it may reinsert the message back into the
    /// end of the queue.
    ///
    /// When NACKed, consumers of this queue will process the message again at some point.
    async fn nack(self) -> Result<(), QueueError>;
}

/// This trait is implemented for the sending/transmitting/producing side of the queue -- the side
/// which takes a payload, and sends it such that it is eventually received by a matching impl of
/// [`TaskQueueReceive`].
///
/// It contains one method: [`TaskQueueSend::send`] which does as expected.
#[async_trait]
pub trait TaskQueueSend<T>: Send + Sync {
    /// Sends a payload through the queue associated with this producer.
    async fn send(&self, payload: T) -> Result<(), QueueError>;
}

/// This trait is implemented for the receiving/consuming side of the queue -- the side which
/// receives batches of [`Delivery`] instances.
///
/// NOTE that some backends don't support batching by default, in which case they will attempt to
/// read until the timeout duration is met or the batch is full.
///
/// It contains one method [`TaskQueueReceive::receive_all`] which does as expected.
#[async_trait]
pub trait TaskQueueReceive<T: Send + Sync, D: Delivery<T>>: Send + Sync {
    /// Receives a batch of deliveries from the queue.
    ///
    /// This function will not return an empty [`Vec`], instead it will await the moment of at least
    /// one event being received to return.
    ///
    /// After the inital event is received (in a queue that does not natively support batching), the
    /// queue is read from in a loop until either the maximum batch size has been met or the timeout
    /// duration has been exceeded.
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<D>, QueueError>;
}
