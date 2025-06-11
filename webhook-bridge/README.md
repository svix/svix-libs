# Svix Webhook Bridge

This service subscribes to a queue or stream and forwards each item to Svix when a valid message is found.

## Usage

```
svix-webhook-bridge -c path/to/svix-webhook-bridge.yaml
```

## Configuration

The CLI itself exposes only a single flag (`-c`, `--cfg`) used to set the path for the config file.
The location of the config file can also be set with the `SVIX_WEBHOOK_BRIDGE_CFG` env var.
The config file itself does the heavy lifting.

When unset, the current working directory is checked for a file named `svix-webhook-bridge.yaml`.

> For an annotated sample configuration see [the example config](svix-webhook-bridge.example.yaml).

`svix-webhook-bridge` is organized in terms of "plugins" which are tasks that run in tandem.
Each plugin represents a unit of work performed while the service runs.

Presently there are 2 "plugins" available for `svix-webhook-bridge`.

### svix-webhook-bridge-plugin-queue-consumer

This plugin consumes messages from message queues to and forwards them to Svix to create messages.

Currently this supports the following messaging systems:
- GCP Pub/Sub
- RabbitMQ
- Redis
- SQS

Generally instances of this plugin are configured in terms of inputs, _optional transformations_, and outputs, where
the input configuration varies by the messaging system.

The output options control how the Svix client is built and configured.
The sole required field is `token`.

The optional _transformation_ can be set to a JavaScript fragment which can be used to reshape the messages as they flow through.

```yaml

plugins:
- type: ...
  input:
    # ... snip ...

  # Reshape the messages we get from the queue before they get sent to Svix
  transformation: | 
    function handler(input) {
      return {
        app_id: input.key,
        message: {
          eventType: input.event_type,
          payload: input.data
        }
      };
    }

  output:
    # ... snip ...
```

Transformations should define a function called `handler` that accepts an object and returns an object.

Messages received by these consumers must follow an expected format:

```
{
    // This indicates which Svix application to send the message to
    "app_id": "app_XYZ",
    
    // The `message` field has the same requirements as the standard `MessageIn`
    // used for Create Message API requests
    "message": {
        "eventType": "my.event",
        "payload": {"abc": 123}
    }
}
```

> The comments in the above JSON are for illustrative purposes only ;)
> That's not valid JSON! Sorry!


For detail on the `message` field, see: <https://api.svix.com/docs#tag/Message/operation/v1.message.create>

Important to note that queues, exchanges, topics, or what have you, should be created and configured independently,
prior to using the plugin. There's nothing in place to automatically create these resources.
The plugin will only try (and fail) to read from the stream in such a case.


#### Example GCP Pub/Sub consumer

The GCP consumer plugin can optionally specify a path to a credentials file.

When left unset, it falls back to looking env vars:
- `GOOGLE_APPLICATION_CREDENTIALS` set to a path to a credentials file
- `GOOGLE_APPLICATION_CREDENTIALS_JSON` set to the contents of a credentials file (ie. a blob of JSON)

```yaml
plugins:
- type: "gcppubsubconsumer"
  input:
    subscription_id: "my-subscription"
    # Optional - will fallback to looking at env vars when left unset.
    credentials_file: "/path/to/credentials.json"
  output:
    # Required (the Svix token to use when creating messages with this consumer)
    token: "XYZ"
```

#### Example RabbitMq consumer

```yaml
plugins:
- type: "rabbitmqconsumer"
  input:
    # Required
    uri: "amqp://guest:guest@localhost:5672/%2f"
    # Required
    queue_name: "my-queue"
    # Optional (default: unset, managed by rabbitmq)
    consumer_tag: "my-consumer-001"
    # Optional (default: false)
    requeue_on_nack: true
  output:
    # Required (the Svix token to use when creating messages with this consumer)
    token: "XYZ"
```

#### Example Redis consumer

```yaml
plugins:
- type: "redisconsumer"
  input:
    # Required
    dsn: "redis://localhost:6379/"
    # Required
    queue_key: "my_queue"
    # Required
    consumer_name: "my_consumer"
    # Required
    consumer_group: "my_group"
    # Required
    max_connections: 4
    # Optional (default: false)
    requeue_on_nack: true
  output:
    # Required (the Svix token to use when creating messages with this consumer)
    token: "XYZ"
```

#### Example SQS consumer

Note that the SQS consumer requires credentials to be set as environment vars:
- `AWS_DEFAULT_REGION`
- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`

> This incidentally means all SQS consumers configured for a given `svix-webhook-bridge` will need to share these details.

```yaml
plugins:
- type: "sqsconsumer"
  input:
    # Required
    queue_dsn: "http://localhost:19324/000000000000/local"
    # Optional (default: false)
    override_endpoint: true
  output:
    # Required (the Svix token to use when creating messages with this consumer)
    token: "XYZ"
```


### svix-webhook-bridge-plugin-webhook-receiver

This plugin starts an HTTP server which accepts webhooks and forwards them to one of the supported messaging
systems.

Again, same as with `svix-webhook-bridge-plugin-queue-consumer`, the supported systems are:

- GCP Pub/Sub
- RabbitMQ
- Redis
- SQS

The HTTP server also (optionally) performs validation of the webhooks using Svix's signature verification method.

The `verification` section for each route can be set one of two ways:
* `none` which accepts and forwards any JSON POST HTTP request.
* `svix` that takes a Svix endpoint secret (starting with `whsec_`) and
  validating it using an official Svix library


Each instance of this plugin can forward requests to one or more messaging destinations based on the trailing path
segment:

```
/webhook/:name
```

#### Example

```yaml
plugins:
- type: "webhookreceiver"
  listen_addr: "0.0.0.0:5000"
  routes:
    - name: "goog"
      verification:
        type: "svix"
        secret: "whsec_XXXXX="
      destination:
        type: gcppubsub
        topic: "example"
    - name: "amz"
      verification:
        type: "none"
      destination:
        # Note that the SQS forwarder requires credentials to be set as environment vars:
        # - `AWS_DEFAULT_REGION`
        # - `AWS_ACCESS_KEY_ID`
        # - `AWS_SECRET_ACCESS_KEY`
        type: "sqs"
        queue_dsn: "https://example.aws.com/my-queue"
```

In this situation, `POST`ing a JSON payload to `http://localhost:5000/webhook/goog` would forward the body
to the `example` topic in GCP Pub/Sub _only when verification passes_, whereas `POST`'ing to
`http://localhost:5000/webhook/amz` will forward the body to SQS without extra validation.
