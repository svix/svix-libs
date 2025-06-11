//! Requires a rabbitmq node to be running on localhost:5672 (the default port) and using the
//! default guest/guest credentials.
//! Try using the `testing-docker-compose.yml` in the repo root to get this going.

use generic_queue::rabbitmq::FieldTable;
use lapin::{options::QueueDeclareOptions, Channel, Connection, ConnectionProperties, Queue};
use serde_json::json;
use std::time::Duration;
use svix::api::MessageIn;
use svix_webhook_bridge_plugin_queue_consumer::{
    config::{OutputOpts, RabbitMqInputOpts, SvixOptions},
    CreateMessageRequest, RabbitMqConsumerConfig, RabbitMqConsumerPlugin,
};
use svix_webhook_bridge_types::{JsReturn, Plugin, TransformerJob};
use wiremock::matchers::{body_partial_json, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn get_test_plugin(
    svix_url: String,
    mq_uri: &str,
    queue_name: &str,
    use_transformation: bool,
) -> RabbitMqConsumerPlugin {
    RabbitMqConsumerPlugin::new(RabbitMqConsumerConfig {
        input: RabbitMqInputOpts {
            uri: mq_uri.to_string(),
            queue_name: queue_name.to_string(),
            ..Default::default()
        },
        transformation: if use_transformation {
            // The actual script doesn't matter since the test case will be performing the
            // transformation, not the actual JS executor.
            Some(String::from("export default function (x) { return x; }"))
        } else {
            None
        },
        output: OutputOpts {
            token: "xxxx".to_string(),
            svix_options: Some(SvixOptions {
                server_url: Some(svix_url),
                ..Default::default()
            }),
        },
    })
}

async fn declare_queue(name: &str, channel: &Channel) -> Queue {
    channel
        .queue_declare(
            name,
            QueueDeclareOptions {
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap()
}

async fn mq_connection(uri: &str) -> Connection {
    let options = ConnectionProperties::default()
        .with_connection_name("test".into())
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);
    Connection::connect(uri, options).await.unwrap()
}

async fn publish(channel: &Channel, queue_name: &str, payload: &[u8]) {
    let confirm = channel
        .basic_publish(
            "",
            queue_name,
            Default::default(),
            payload,
            Default::default(),
        )
        .await
        .unwrap();
    confirm.await.unwrap();
}

/// General "pause while we wait for messages to travel" beat. If you're seeing flakes, bump this up.
const WAIT_MS: u64 = 150;
/// These tests assume a "vanilla" rabbitmq instance, using the default port, creds, exchange...
const MQ_URI: &str = "amqp://guest:guest@localhost:5672/%2f";

/// Push a msg on the queue.
/// Check to see if the svix server sees a request.
#[tokio::test]
async fn test_consume_ok() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            "_SVIX_APP_ID": "app_1234",
            "_SVIX_EVENT_TYPE": "testing.things",
            "hi": "there",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), MQ_URI, queue_name, false);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        post_options: None,
    };

    publish(&channel, queue_name, &serde_json::to_vec(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();
    channel
        .queue_delete(queue_name, Default::default())
        .await
        .ok();
}
/// Push a msg on the queue.
/// Check to see if the svix server sees a request, but this time transform the payload.
#[tokio::test]
async fn test_consume_transformed_ok() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .and(body_partial_json(json!({ "payload": { "good": "bye" } })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            "_SVIX_APP_ID": "app_1234",
            "_SVIX_EVENT_TYPE": "testing.things",
            // The adjustment made via the transformation...
            "good": "bye",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let mut plugin = get_test_plugin(mock_server.uri(), MQ_URI, queue_name, true);
    let (transformer_tx, mut transformer_rx) =
        tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = transformer_rx.recv().await {
            let mut out = x.payload;
            // Prune out the "hi" key.
            out["message"]["payload"]
                .as_object_mut()
                .unwrap()
                .remove("hi");
            // Add the "good" key.
            out["message"]["payload"]["good"] = json!("bye");
            x.callback_tx.send(Ok(JsReturn::Object(out))).ok();
        }
    });
    plugin.set_transformer(Some(transformer_tx));

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        post_options: None,
    };

    publish(&channel, queue_name, &serde_json::to_vec(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();
    channel
        .queue_delete(queue_name, Default::default())
        .await
        .ok();
}

#[tokio::test]
async fn test_missing_app_id_nack() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), MQ_URI, queue_name, false);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &channel,
        queue_name,
        &serde_json::to_vec(&json!({
            // No app id
            "message": {
                "eventType": "testing.things",
                "payload": {
                    "hi": "there",
                }
            },

        }))
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;
    handle.abort();
    channel
        .queue_delete(queue_name, Default::default())
        .await
        .ok();
}

#[tokio::test]
async fn test_missing_event_type_nack() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), MQ_URI, queue_name, false);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &channel,
        queue_name,
        &serde_json::to_vec(&json!({
            "app_id": "app_1234",
            "message": {
                // No event type
                "payload": {
                    "hi": "there",
                }
            },
        }))
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;
    handle.abort();
    channel
        .queue_delete(queue_name, Default::default())
        .await
        .ok();
}

/// Check that the plugin keeps running when it can't send a message to svix
#[tokio::test]
async fn test_consume_svix_503() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), MQ_URI, queue_name, false);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &channel,
        queue_name,
        &serde_json::to_vec(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
            post_options: None,
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(!handle.is_finished());
    handle.abort();
    channel
        .queue_delete(queue_name, Default::default())
        .await
        .ok();
}

/// Check that the plugin keeps running when it can't send a message to svix because idk, the servers are all offline??
#[tokio::test]
async fn test_consume_svix_offline() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let mock_server = MockServer::start().await;

    let plugin = get_test_plugin(mock_server.uri(), MQ_URI, queue_name, false);

    // bye-bye svix...
    drop(mock_server);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &channel,
        queue_name,
        &serde_json::to_vec(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
            post_options: None,
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(!handle.is_finished());
    handle.abort();
    channel
        .queue_delete(queue_name, Default::default())
        .await
        .ok();
}
