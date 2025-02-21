use crate::types::IntegrationId;
use generic_queue::rabbitmq::{BasicProperties, BasicPublishOptions};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The [`IntegrationConfig`] is the struct associated with a given [`IntegrationId`]. When the route
/// associated with an [`IntegrationId`] receives a webhook, or any other HTTP request, then it will
/// attempt to be validated with the specified [`VerificationScheme`]. Should the configured scheme
/// indicate that the webhook is valid, then the webhook will be forwarded verbatim to the configured
/// [`ForwardDestination`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IntegrationConfig {
    pub name: IntegrationId,
    pub verification: VerificationScheme,
    pub destination: ForwardDestination,
}

/// The [`VerificationScheme`] is an enum with variant for every method for verifying a webhook's
/// authenticity that is supported by this service. As of present, the [`VerificationScheme::Svix`]
/// variant and [`VerificationScheme::JavaScript`] variant are supported.
///
/// Upon an [`IntegrationId`] receiving a webhook, then the configured [`VerificationScheme`]'s
/// associated method will be called returning a simple [`bool`] result on whether the HTTP request
/// received is a valid webhook.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum VerificationScheme {
    /// The [`VerificationScheme::Svix`] variant is a simple scheme which simply uses the official
    /// [`svix`] library and a configured secret to verify webhooks dispatched by Svix.
    Svix {
        #[serde(flatten)]
        secret: SvixSecret,
    },
    None,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GCPPubSubOutputOpts {
    pub topic: String,
    pub credentials_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RabbitMqOutputOpts {
    /// Connection string for RabbitMQ.
    pub uri: String,
    /// The exchange to publish messages to.
    pub exchange: String,
    /// The routing key to publish messages to.
    pub routing_key: String,
    #[serde(default)]
    pub publish_options: BasicPublishOptions,
    #[serde(default)]
    pub publish_properties: BasicProperties,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RedisOutputOpts {
    pub dsn: String,
    pub max_connections: u16,
    pub queue_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SqsOutputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
}

/// The [`ForwardDestination`] is a part of the [`IntegrationConfig`] for every `[IntegrationId`]
/// and defines where the webhook will be redirected upon the [`IntegrationConfig`]'s associated
/// [`VerificationScheme`] deeming that an HTTP request is a valid, authentic webhook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ForwardDestination {
    // FIXME: HTTP forwarding
    GCPPubSub(GCPPubSubOutputOpts),
    RabbitMQ(RabbitMqOutputOpts),
    Redis(RedisOutputOpts),
    SQS(SqsOutputOpts),
}

/// All webhooks dispatched by Svix and all associated metadata are signed before being sent. The
/// key for verifying this signature is associated with a given endpoint registered with Svix and
/// begins with `whsec_`.
///
/// This enum has two variants -- [`SvixSecret::Secret`] which is meant to be the direct secret
/// value, while [`SvixSecret::EnvVar`] is meant to be a valid env var which contains the secret.
///
/// They are distinguished in configuration via setting either `secret` or `secret_env`.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SvixSecret {
    Secret { secret: String },
    EnvVar { secret_env: String },
}

impl SvixSecret {
    pub fn to_secret(&self) -> anyhow::Result<String> {
        match self {
            SvixSecret::Secret { secret } => Ok(secret.clone()),
            SvixSecret::EnvVar { secret_env } => {
                if let Ok(secret) = std::env::var(secret_env) {
                    Ok(secret)
                } else {
                    anyhow::bail!("env var {secret_env} not set or invalid");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SvixSecret, VerificationScheme};
    use crate::WebhookReceiverPluginConfig;

    #[test]
    fn test_svix_secret_parsing() {
        // FIXME: destinations need fixups
        let config_text = r#"
                listen_addr: "0.0.0.0:1234"
                routes:
                  - name: "path-1"
                    verification:
                      type: svix
                      secret: whsec_bm90IHJlYWw=
                    destination:
                      type: rabbitmq
                      uri: amqp://guest:guest@localhost:5672/%2f
                      exchange: "myexhange"
                      routing_key: ""
                  - name: "path_2"
                    verification:
                      type: svix
                      secret_env: SECRET_SVIX_TOKEN
                    destination:
                      type: sqs
                      queue_dsn: http://localhost:9324/queue/my-queue
            "#;

        let config: WebhookReceiverPluginConfig = serde_yaml::from_str(config_text).unwrap();

        let configured_secret = "whsec_bm90IHJlYWw=".to_owned();
        let set_secret_env = "whsec_invalid".to_owned();

        assert!(matches!(
            &config.routes[0].verification,
            VerificationScheme::Svix {
                secret: SvixSecret::Secret { .. }
            }
        ));
        assert!(matches!(
            &config.routes[1].verification,
            VerificationScheme::Svix {
                secret: SvixSecret::EnvVar { .. }
            }
        ));

        std::env::set_var("SECRET_SVIX_TOKEN", &set_secret_env);

        let secret = match &config.routes[0].verification {
            VerificationScheme::Svix { secret } => secret,
            _ => panic!("unexpected verification scheme"),
        };
        assert_eq!(secret.to_secret().unwrap(), configured_secret);

        let secret = match &config.routes[1].verification {
            VerificationScheme::Svix { secret } => secret,
            _ => panic!("unexpected verification scheme"),
        };
        assert_eq!(secret.to_secret().unwrap(), set_secret_env);
    }
}
