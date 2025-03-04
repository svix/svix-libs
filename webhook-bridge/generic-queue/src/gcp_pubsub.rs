//! Support for Google Cloud Pub/Sub.
//!
//! In this system subscriptions are like queue bindings to topics.
//! Consumers need a subscription id to start receiving messages.
//! We don't have any public API for managing/creating/deleting subscriptions in this module, so
//! this is left to the user to do via whatever method they like.
//!
//! - <https://cloud.google.com/pubsub/docs/create-topic>
//! - <https://cloud.google.com/pubsub/docs/create-subscription#pubsub_create_push_subscription-gcloud>
//! - <https://cloud.google.com/pubsub/docs/publisher> (how to publish messages ad hoc, helpful for debugging)
//!
//! Don't have a better place to mention this just yet.
//! When testing against the gcloud emulator, you need to set `PUBSUB_EMULATOR_HOST` to the bind
//! address, and `PUBSUB_PROJECT_ID` (matching however the emulator was configured).
//! This should bypass the need for credentials and so on.
//! ```sh
//! export PUBSUB_EMULATOR_HOST=localhost:8085
//! export PUBSUB_PROJECT_ID=local-project
//! ```
//! > N.b. the rust client hardcodes the project id to `local-project` when it sees the
//! > `PUBSUB_EMULATOR_HOST` env var in use, so if you see errors about resources not found etc, it
//! > might be because of a project mismatch.
//!
//! To use the `gcloud` CLI with the emulator (useful for creating topics/subscriptions), you have
//! to configure an override for the pubsub API:
//!
//! ```sh
//! gcloud config set api_endpoint_overrides/pubsub "http://${PUBSUB_EMULATOR_HOST}/"
//! ```
//! Note that you'll also have to manually set it back to the default as needed:
//! ```sh
//! gcloud config unset api_endpoint_overrides/pubsub
//! ```
//! h/t <https://stackoverflow.com/a/73059126>
//!
//! Also note, and this is odd, `gcloud` will prompt you to login even though you're trying to
//! connect to a local process.
//! Go ahead and follow the prompts to get your CLI working.
//!
//! I guess it still wants to talk to GCP for other interactions other than the pubsub API.
//!
//! ## Example `gcloud` usage:
//! ```sh
//! gcloud --project=local-project pubsub topics create tester
//! gcloud --project=local-project pubsub topics create dead-letters
//! gcloud --project=local-project pubsub subscriptions create local-1 \
//!   --topic=tester \
//!   --dead-letter-topic=dead-letters \
//!   --max-delivery-attempts=5
//! gcloud --project local-project pubsub topics publish tester --message='{"my message": 1234}'
//! ```
//!
use crate::{Delivery, QueueError, TaskQueueBackend, TaskQueueReceive, TaskQueueSend};
use async_trait::async_trait;
use futures_util::StreamExt;
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_default::WithAuthExt;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscriber::ReceivedMessage;
use google_cloud_pubsub::subscription::Subscription;
use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{marker::PhantomData, time::Duration};

pub struct GCPPubSubConfig {
    pub topic: String,
    pub subscription_id: String,
    pub credentials_file: Option<PathBuf>,
}

pub struct GCPPubSubQueueBackend;

/// Make a `ClientConfig` from a `CredentialsFile` on disk.
async fn configure_client_from_file<P: AsRef<Path>>(
    cred_file_path: P,
) -> Result<ClientConfig, QueueError> {
    let bytes = std::fs::read(cred_file_path).map_err(QueueError::generic)?;
    let creds: CredentialsFile = serde_json::from_slice(&bytes).map_err(QueueError::generic)?;
    ClientConfig::default()
        .with_credentials(creds)
        .await
        .map_err(QueueError::generic)
}

/// Making a `ClientConfig` via env vars is possible in two ways:
/// - setting `GOOGLE_APPLICATION_CREDENTIALS` to the file path to have it loaded automatically
/// - setting `GOOGLE_APPLICATION_CREDENTIALS_JSON` to the file contents (avoiding the need for a
///   file on disk).
///
/// Naturally relying on env vars for configuration means it's difficult to have
async fn configure_client_from_env() -> Result<ClientConfig, QueueError> {
    ClientConfig::default()
        .with_auth()
        .await
        .map_err(QueueError::generic)
}

async fn get_client(cfg: &GCPPubSubConfig) -> Result<Client, QueueError> {
    let config = {
        if let Some(fp) = &cfg.credentials_file {
            tracing::trace!("reading gcp creds from file: {}", fp.display());
            configure_client_from_file(&fp).await?
        } else {
            tracing::trace!("reading gcp creds from env");
            configure_client_from_env().await?
        }
    };
    Client::new(config).await.map_err(QueueError::generic)
}

async fn get_consumer(
    client: Client,
    cfg: &GCPPubSubConfig,
) -> Result<GCPPubSubQueueConsumer, QueueError> {
    Ok(GCPPubSubQueueConsumer {
        client,
        subscription_id: cfg.subscription_id.clone(),
    })
}
async fn get_producer(
    client: Client,
    cfg: &GCPPubSubConfig,
) -> Result<GCPPubSubQueueProducer, QueueError> {
    let topic = client.topic(&cfg.topic);
    // Only warn on startup, if the topic doesn't exist. If it gets created after the fact, we
    // should be able to still use it when available, otherwise if it's still missing at that time, error.
    if !topic.exists(None).await.map_err(QueueError::generic)? {
        tracing::warn!("topic {} does not exist", &cfg.topic);
    }
    Ok(GCPPubSubQueueProducer {
        client,
        topic: cfg.topic.clone(),
    })
}

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync> TaskQueueBackend<T>
    for GCPPubSubQueueBackend
{
    type PairConfig = GCPPubSubConfig;
    type Delivery = GCPPubSubDelivery<T>;
    type Producer = GCPPubSubQueueProducer;
    type Consumer = GCPPubSubQueueConsumer;

    async fn new_pair(
        cfg: GCPPubSubConfig,
    ) -> Result<(GCPPubSubQueueProducer, GCPPubSubQueueConsumer), QueueError> {
        let client = get_client(&cfg).await?;
        let producer = get_producer(client.clone(), &cfg).await?;
        let consumer = get_consumer(client, &cfg).await?;
        Ok((producer, consumer))
    }

    async fn producing_half(cfg: GCPPubSubConfig) -> Result<GCPPubSubQueueProducer, QueueError> {
        let client = get_client(&cfg).await?;
        let producer = get_producer(client, &cfg).await?;
        Ok(producer)
    }

    async fn consuming_half(cfg: GCPPubSubConfig) -> Result<GCPPubSubQueueConsumer, QueueError> {
        let client = get_client(&cfg).await?;
        let consumer = get_consumer(client, &cfg).await?;
        Ok(consumer)
    }
}

pub struct GCPPubSubDelivery<T: DeserializeOwned> {
    message: ReceivedMessage,
    _pd: PhantomData<T>,
}

impl<T: DeserializeOwned> std::fmt::Debug for GCPPubSubDelivery<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GCPPubSubDelivery")
            .field("ack_id", &self.message.ack_id())
            .field("message_id", &self.message.message.message_id)
            .finish()
    }
}

#[async_trait]
impl<T: DeserializeOwned + Send + Serialize + Sync> Delivery<T> for GCPPubSubDelivery<T> {
    fn payload(&self) -> Result<T, QueueError> {
        serde_json::from_slice(&self.message.message.data).map_err(Into::into)
    }

    async fn ack(self) -> Result<(), QueueError> {
        self.message.ack().await.map_err(QueueError::generic)
    }

    async fn nack(self) -> Result<(), QueueError> {
        self.message.nack().await.map_err(QueueError::generic)
    }
}

pub struct GCPPubSubQueueProducer {
    client: Client,
    topic: String,
}

impl std::fmt::Debug for GCPPubSubQueueProducer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GCPPubSubQueueProducer")
            .field("topic", &self.topic)
            .finish()
    }
}

#[async_trait]
impl<T: 'static + Serialize + Send + Sync> TaskQueueSend<T> for GCPPubSubQueueProducer {
    async fn send(&self, payload: T) -> Result<(), QueueError> {
        let msg = PubsubMessage {
            data: serde_json::to_string(&payload)?.into(),
            ..Default::default()
        };

        // N.b. defer the creation of a publisher/topic until needed. Helps recover when
        // the topic does not yet exist, but will soon.
        // Might be more expensive to recreate each time, but overall more reliable.
        let topic = self.client.topic(&self.topic);

        // Publishing to a non-existent topic will cause the publisher to wait (forever?)
        // Giving this error will allow dependents like `svix-webhook-bridge-plugin-webhook-receiver` to
        // respond 500 immediately when this happens, instead of holding the connection open
        // indefinitely.
        if !topic.exists(None).await.map_err(QueueError::generic)? {
            return Err(QueueError::Generic(
                format!("topic {} does not exist", &topic.id()).into(),
            ));
        }
        // FIXME: may need to expose `PublisherConfig` to caller so they can tweak this
        let publisher = topic.new_publisher(None);
        let awaiter = publisher.publish(msg).await;
        awaiter.get().await.map_err(QueueError::generic)?;
        Ok(())
    }
}

pub struct GCPPubSubQueueConsumer {
    client: Client,
    subscription_id: String,
}

impl GCPPubSubQueueConsumer {
    async fn subscription(&self) -> Result<Subscription, QueueError> {
        let subscription = self.client.subscription(&self.subscription_id);
        if !subscription
            .exists(None)
            .await
            .map_err(QueueError::generic)?
        {
            return Err(QueueError::Generic(
                format!("subscription {} does not exist", &self.subscription_id).into(),
            ));
        }
        Ok(subscription)
    }
}

impl std::fmt::Debug for GCPPubSubQueueConsumer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GCPPubSubQueueConsumer")
            .field("subscription_id", &self.subscription_id)
            .finish()
    }
}

#[async_trait]
impl<T: 'static + DeserializeOwned + Send + Serialize + Sync>
    TaskQueueReceive<T, GCPPubSubDelivery<T>> for GCPPubSubQueueConsumer
{
    async fn receive_all(
        &mut self,
        max_batch_size: usize,
        timeout: Duration,
    ) -> Result<Vec<GCPPubSubDelivery<T>>, QueueError> {
        let start = Instant::now();
        let subscription = self.subscription().await?;
        let mut stream = subscription
            .subscribe(None)
            .await
            .map_err(QueueError::generic)?;

        let mut out = Vec::with_capacity(max_batch_size);

        loop {
            if let Some(message) = stream.next().await {
                tracing::trace!(
                    "Got Message: id={}, ack_id={} ",
                    &message.message.message_id,
                    &message.ack_id()
                );

                out.push(GCPPubSubDelivery {
                    message,
                    _pd: PhantomData,
                });
            }

            if out.len() >= max_batch_size || (!out.is_empty() && start.elapsed() > timeout) {
                break;
            }
        }
        Ok(out)
    }
}
