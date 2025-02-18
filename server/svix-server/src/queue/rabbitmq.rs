use lapin::{
    options::{BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable},
    ConnectionProperties,
};
use omniqueue::backends::{RabbitMqBackend, RabbitMqConfig};
use svix_ksuid::{KsuidLike, KsuidMs};

use super::{TaskQueueConsumer, TaskQueueProducer};
use crate::error::{Result, Traceable};

/// Returns a new_pair producers/consumers that use RabbitMQ under the hood.
///
/// USE WITH CAUTION - For the time being, this implementation has only been exercised in local development
/// and testing environments. There may be production kinks that need working out
pub async fn new_pair(
    dsn: &str,
    queue_name: String,
    prefetch_size: u16,
) -> Result<(TaskQueueProducer, TaskQueueConsumer)> {
    let conn = lapin::Connection::connect(dsn, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await.unwrap();

    let exchange_name = declare_delayed_message_exchange(&channel).await.trace()?;
    declare_bound_queue(&queue_name, &exchange_name, &channel)
        .await
        .trace()?;

    drop(channel);

    // Ref https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.consume.consumer-tag
    let consumer_tag = format!(
        "{queue_name}-consumer-{}",
        // prevent possible errors around duplicate consumer tags
        KsuidMs::new(None, None).to_string()
    );

    let (producer, consumer) = RabbitMqBackend::builder(RabbitMqConfig {
        uri: dsn.to_owned(),
        connection_properties: Default::default(),
        publish_exchange: "first-message".to_owned(),
        publish_routing_key: queue_name.clone(),
        publish_options: Default::default(),
        publish_properties: Default::default(),
        consume_queue: queue_name,
        consumer_tag,
        consume_options: BasicConsumeOptions {
            // https://www.rabbitmq.com/amqp-0-9-1-reference.html#domain.no-local
            // false because I don't care if the same connection reads and publishes to the same
            // queue
            no_local: false,

            // https://www.rabbitmq.com/amqp-0-9-1-reference.html#domain.no-ack
            // Obviously want message ACKs to ensure message are handled
            no_ack: false,

            // https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.consume.exclusive
            // More than one worker should be able to read from the same queue
            exclusive: false,

            // https://www.rabbitmq.com/amqp-0-9-1-reference.html#domain.no-wait
            // want the server to respond if there's a failure
            nowait: false,
        },
        consume_arguments: Default::default(),
        // Ref https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.qos.prefetch-size
        //
        // prefetch_size tells the consumer how many messages to load in batches from the queue.
        // Higher values generally means better queue performance (fewer messages stuck in the
        // queue), at the cost of consumer memory.
        // Additionally, if the prefetch_size is *too* large, a single worker can "starve" other
        // workers, potentially hurting total message throughput.
        //
        // "global" enforces the same limit for other consumers on the channel, which isn't
        // necessarily what we want
        consume_prefetch_count: Some(prefetch_size),
        requeue_on_nack: true,
    })
    .build_pair()
    .await
    .expect("Error initializing rabbitmq queue");

    let producer = TaskQueueProducer::new(producer);
    let consumer = TaskQueueConsumer::new(consumer);

    Ok((producer, consumer))
}

async fn declare_delayed_message_exchange(channel: &lapin::Channel) -> Result<String> {
    let exchange_name = "first-message";

    // See https://www.rabbitmq.com/amqp-0-9-1-reference.html#exchange.declare
    let opts = ExchangeDeclareOptions {
        // Want the server to create the exchange if it doesn't already exist
        passive: false,
        nowait: false,
        // Want the exchange to survive restarts, and not be deleted even when the underlying queues are
        // deleted
        durable: true,
        auto_delete: false,
        internal: false,
    };

    // See https://github.com/rabbitmq/rabbitmq-delayed-message-exchange#usage
    let mut args = FieldTable::default();
    args.insert(
        "x-delayed-type".into(),
        AMQPValue::LongString("direct".into()),
    );

    channel
        .exchange_declare(
            exchange_name,
            lapin::ExchangeKind::Custom("x-delayed-message".into()),
            opts,
            args,
        )
        .await?;

    Ok(exchange_name.to_owned())
}

async fn declare_bound_queue(
    queue_name: &str,
    exchange_name: &str,
    channel: &lapin::Channel,
) -> Result<()> {
    // Ref https://www.rabbitmq.com/amqp-0-9-1-quickref.html#queue.declare
    let opts = QueueDeclareOptions {
        // If the queue already exists with the same configuration, we want the server to respond w/OK
        // Alternatively, if the queue exists with a _different_ configuration, we want this to fail
        passive: false,
        nowait: false,

        // We want to support multiple consumers per queue
        exclusive: false,

        // We want the queue to survive rust-primary restarts
        durable: true,
        auto_delete: false,
    };

    // Refs https://www.rabbitmq.com/maxlength.html#definition-using-x-args and https://www.rabbitmq.com/dlx.html#using-optional-queue-arguments
    // We may want to figure out what the queue length enforcement looks like and dead letter queueing at a later point in time
    let args = FieldTable::default();
    channel.queue_declare(queue_name, opts, args).await?;

    let routing_key = queue_name;

    channel
        .queue_bind(
            queue_name,
            exchange_name,
            routing_key,
            QueueBindOptions { nowait: false },
            FieldTable::default(),
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use futures::StreamExt as _;
    use lapin::options::{BasicConsumeOptions, BasicQosOptions};
    use svix_ksuid::{KsuidLike as _, KsuidMs};

    use crate::{cfg, queue::QueueTask};

    #[tokio::test]
    // run with `cargo test -- --ignored rabbitmq` only when rabbitmq is up and configured
    #[ignore]
    async fn test_messages_have_ids() {
        const QUEUE_NAME: &str = "test_messages_have_ids_q";

        let cfg = cfg::load().expect("Error loading configuration");
        let cfg::QueueBackend::RabbitMq(dsn) = cfg.queue_backend() else {
            panic!("This test must only run when the rabbitmq backend is enabled");
        };
        let prefetch_size = cfg.rabbit_consumer_prefetch_size.unwrap_or(1);

        // Send message with omniqueue
        {
            let (producer, _) = super::new_pair(dsn, QUEUE_NAME.to_owned(), prefetch_size)
                .await
                .unwrap();

            producer.send(&QueueTask::HealthCheck, None).await.unwrap();
        }

        // Receive with lapin consumer
        {
            let conn = lapin::Connection::connect(dsn, lapin::ConnectionProperties::default())
                .await
                .unwrap();
            let channel = conn.create_channel().await.unwrap();

            let consumer_tag = format!(
                "{QUEUE_NAME}-consumer-{}",
                KsuidMs::new(None, None).to_string()
            );

            let opts = BasicConsumeOptions {
                no_local: false,
                no_ack: false,
                exclusive: false,
                nowait: false,
            };

            channel
                .basic_qos(prefetch_size, BasicQosOptions { global: false })
                .await
                .unwrap();

            let mut consumer = channel
                .basic_consume(QUEUE_NAME, &consumer_tag, opts, Default::default())
                .await
                .unwrap();

            let delivery = consumer.next().await.unwrap().unwrap();

            // ... to assert that there is a message ID
            delivery.properties.message_id().as_ref().unwrap();
        }
    }
}
