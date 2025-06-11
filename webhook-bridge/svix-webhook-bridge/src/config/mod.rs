use serde::Deserialize;
use svix_webhook_bridge_types::Plugin;
use tracing::Level;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,
    /// The log level to run the service with. Supported: info, debug, trace
    #[serde(default)]
    pub log_level: LogLevel,
    /// The log format that all output will follow. Supported: default, json
    #[serde(default)]
    pub log_format: LogFormat,
    /// The OpenTelemetry service name to use
    pub opentelemetry_service_name: Option<String>,
    /// The OpenTelemetry address to send events to if given.
    pub opentelemetry_address: Option<String>,
    /// The ratio at which to sample spans when sending to OpenTelemetry. When not given it defaults
    /// to always sending. If the OpenTelemetry address is not set, this will do nothing.
    pub opentelemetry_sample_ratio: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Info => Level::INFO,
            Self::Debug => Level::DEBUG,
            Self::Trace => Level::TRACE,
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Default,
    Json,
}

// FIXME: ideally we wouldn't need to modify `ConsumerConfig` when adding new plugins.
//   Possibly we could codegen this type via macro that look at a data/cfg file, or in
//   a build script.
#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum PluginConfig {
    #[cfg(feature = "gcp-pubsub")]
    GCPPubSubConsumer(svix_webhook_bridge_plugin_queue_consumer::GCPPubSubConsumerConfig),

    #[cfg(feature = "rabbitmq")]
    RabbitMQConsumer(svix_webhook_bridge_plugin_queue_consumer::RabbitMqConsumerConfig),

    #[cfg(feature = "redis")]
    RedisConsumer(svix_webhook_bridge_plugin_queue_consumer::RedisConsumerConfig),

    #[cfg(feature = "sqs")]
    SqsConsumer(svix_webhook_bridge_plugin_queue_consumer::SqsConsumerConfig),

    #[cfg(feature = "webhook-receiver")]
    WebhookReceiver(svix_webhook_bridge_plugin_webhook_receiver::WebhookReceiverPluginConfig),

    #[serde(other)]
    Unknown,
}

impl TryInto<Box<dyn Plugin>> for PluginConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        match self {
            #[cfg(feature = "gcp-pubsub")]
            PluginConfig::GCPPubSubConsumer(cc) => cc.try_into(),

            #[cfg(feature = "rabbitmq")]
            PluginConfig::RabbitMQConsumer(cc) => cc.try_into(),

            #[cfg(feature = "redis")]
            PluginConfig::RedisConsumer(cc) => cc.try_into(),

            #[cfg(feature = "sqs")]
            PluginConfig::SqsConsumer(cc) => cc.try_into(),

            #[cfg(feature = "webhook-receiver")]
            PluginConfig::WebhookReceiver(cc) => cc.try_into(),

            PluginConfig::Unknown => Err("unknown plugin"),
        }
    }
}
