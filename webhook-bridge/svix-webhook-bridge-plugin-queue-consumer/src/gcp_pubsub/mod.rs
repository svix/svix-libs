use crate::config::{GCPPubSubConsumerConfig, GCPPubSubInputOpts};
use crate::error::Error;
use crate::PLUGIN_VERS;
use crate::{create_svix_message, CreateMessageRequest};
use crate::{run_inner, Consumer, PLUGIN_NAME};
use generic_queue::gcp_pubsub::{
    GCPPubSubConfig, GCPPubSubDelivery, GCPPubSubQueueBackend, GCPPubSubQueueConsumer,
};
use generic_queue::{Delivery, TaskQueueBackend, TaskQueueReceive};
use std::time::Duration;
use svix::api::Svix;
use svix_webhook_bridge_types::{async_trait, JsObject, Plugin, TransformerTx};
use tracing::instrument;

pub struct GCPPubSubConsumerPlugin {
    input_options: GCPPubSubInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<String>,
}

impl GCPPubSubConsumerPlugin {
    pub fn new(
        GCPPubSubConsumerConfig {
            input,
            transformation,
            output,
        }: GCPPubSubConsumerConfig,
    ) -> Self {
        Self {
            input_options: input,
            svix_client: Svix::new(output.token, output.svix_options.map(Into::into)),
            transformer_tx: None,
            transformation,
        }
    }

    /// Pulls N messages off the queue and feeds them to [`Self::process`].
    #[instrument(skip_all,
        fields(
            otel.kind = "CONSUMER",
            messaging.system = "gcp-pubsub",
            messaging.operation = "receive",
            messaging.source = &self.input_options.subscription_id,
            svixagent_plugin.name = PLUGIN_NAME,
            svixagent_plugin.vers = PLUGIN_VERS,
        )
    )]
    async fn receive(&self, consumer: &mut GCPPubSubQueueConsumer) -> std::io::Result<()> {
        let deliveries = consumer
            .receive_all(1, Duration::from_millis(10))
            .await
            .map_err(Error::from)?;
        tracing::trace!("received: {}", deliveries.len());
        for delivery in deliveries {
            self.process(delivery).await?;
        }
        Ok(())
    }

    /// Parses the delivery as JSON and feeds it into [`create_svix_message`].
    /// Will nack the delivery if either the JSON parse, transformation, or the request to svix fails.
    #[instrument(skip_all, fields(messaging.operation = "process"))]
    async fn process(&self, delivery: GCPPubSubDelivery<JsObject>) -> std::io::Result<()> {
        let payload = match Delivery::<JsObject>::payload(&delivery) {
            Ok(p) => p,
            Err(e) => {
                tracing::warn!("{e}");
                delivery.nack().await.map_err(Error::from)?;
                return Ok(());
            }
        };

        let payload = if let Some(script) = &self.transformation {
            match self.transform(script.clone(), payload).await {
                Err(e) => {
                    tracing::error!("nack: {e}");
                    delivery.nack().await.map_err(Error::from)?;
                    return Ok(());
                }
                Ok(x) => x,
            }
        } else {
            payload
        };

        match create_svix_message(&self.svix_client, payload).await {
            Ok(_) => {
                tracing::trace!("ack");
                delivery.ack().await.map_err(Error::from)?
            }
            Err(e) => {
                tracing::error!("nack: {e}");
                delivery.nack().await.map_err(Error::from)?
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Consumer for GCPPubSubConsumerPlugin {
    fn transformer_tx(&self) -> Option<&TransformerTx> {
        self.transformer_tx.as_ref()
    }
    async fn consume(&self) -> std::io::Result<()> {
        let mut consumer =
            <GCPPubSubQueueBackend as TaskQueueBackend<CreateMessageRequest>>::consuming_half(
                GCPPubSubConfig {
                    subscription_id: self.input_options.subscription_id.clone(),
                    credentials_file: self.input_options.credentials_file.clone(),
                    // Topics are for producers so we don't care
                    topic: String::new(),
                },
            )
            .await
            .map_err(Error::from)?;
        tracing::debug!(
            "gcp pubsub consuming: {}",
            &self.input_options.subscription_id
        );

        loop {
            self.receive(&mut consumer).await?;
        }
    }
}

impl TryInto<Box<dyn Plugin>> for GCPPubSubConsumerConfig {
    type Error = &'static str;
    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(GCPPubSubConsumerPlugin::new(self)))
    }
}

#[async_trait]
impl Plugin for GCPPubSubConsumerPlugin {
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }
    async fn run(&self) -> std::io::Result<()> {
        run_inner(self, "gcp subsub", &self.input_options.subscription_id).await
    }
}
