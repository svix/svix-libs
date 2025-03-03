use generic_queue::rabbitmq::{BasicConsumeOptions, FieldTable};
use serde::Deserialize;
use std::path::PathBuf;
use svix::api::SvixOptions as _SvixOptions;

#[derive(Debug, Default, Deserialize)]
pub struct RabbitMqConsumerConfig {
    pub input: RabbitMqInputOpts,
    pub transformation: Option<String>,
    pub output: OutputOpts,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedisConsumerConfig {
    pub input: RedisInputOpts,
    pub transformation: Option<String>,
    pub output: OutputOpts,
}

#[derive(Debug, Default, Deserialize)]
pub struct SqsConsumerConfig {
    pub input: SqsInputOpts,
    pub transformation: Option<String>,
    pub output: OutputOpts,
}

#[derive(Debug, Default, Deserialize)]
pub struct GCPPubSubConsumerConfig {
    pub input: GCPPubSubInputOpts,
    pub transformation: Option<String>,
    pub output: OutputOpts,
}

// N.b. the codegen types we get from openapi don't impl Deserialize so we need our own version.
#[derive(Debug, Default, Deserialize)]
pub struct SvixOptions {
    #[serde(default)]
    pub debug: bool,
    pub server_url: Option<String>,
}

impl From<SvixOptions> for _SvixOptions {
    fn from(SvixOptions { debug, server_url }: SvixOptions) -> Self {
        _SvixOptions { debug, server_url }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct OutputOpts {
    /// Svix API token for the client.
    pub token: String,
    /// Options for the Svix client.
    pub svix_options: Option<SvixOptions>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RabbitMqInputOpts {
    /// Connection string for RabbitMQ.
    pub uri: String,
    /// The name of the queue to consume from.
    /// N.b. the queue must be declared before the consumer can connect to it.
    pub queue_name: String,
    /// Identifier for the consumer.
    pub consumer_tag: Option<String>,

    pub consume_opts: Option<BasicConsumeOptions>,
    pub consume_args: Option<FieldTable>,

    pub requeue_on_nack: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedisInputOpts {
    pub dsn: String,
    pub max_connections: u16,
    pub reinsert_on_nack: bool,
    pub queue_key: String,
    pub consumer_group: String,
    pub consumer_name: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct SqsInputOpts {
    pub queue_dsn: String,
    pub override_endpoint: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct GCPPubSubInputOpts {
    pub subscription_id: String,
    pub credentials_file: Option<PathBuf>,
}
