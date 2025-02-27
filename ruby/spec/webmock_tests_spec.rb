require "spec_helper"
require "svix"

require_relative "./json_responses"

describe "API Client" do
  let!(:svx) { Svix::Client.new("testsk_token.eu", Svix::SvixOptions.new(false, "http://api.example")) }
  let(:host) { "http://api.example" }

  it "token/user-agent is sent" do
    stub_request(:get, "#{host}/api/v1/app")
      .to_return(
        status: 200,
        body: ListResponseAppOut_JSON
      )

    svx.application.list
    expect(WebMock).to(
      have_requested(:get, "#{host}/api/v1/app")
        .with(
          headers: {
            "Authorization" => "Bearer testsk_token.eu",
            "User-Agent" => "svix-libs/#{Svix::VERSION}/ruby"
          }
        )
    )
  end

  it "test hashtag in query param" do
    stub_request(:get, "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id")
      .with(query: hash_including({}))
      .to_return(
        status: 200,
        body: ListResponseMessageAttemptOut_JSON
      )

    svx.message_attempt.list_by_endpoint("app_id", "endpoint_id", {tag: "#"})

    expect(WebMock).to(have_requested(:get, "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id?tag=%23"))
  end

  it "test Date in query param" do
    stub_request(:get, "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id")
      .with(query: hash_including({}))
      .to_return(
        status: 200,
        body: ListResponseMessageAttemptOut_JSON
      )

    svx.message_attempt.list_by_endpoint("app_id", "endpoint_id", {tag: "#", before: Time.at(1740602143)})

    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id?tag=%23&before=2025-02-26T20%3A35%3A43%2B00%3A00"
      )
    )
  end

  it "test Date request body" do
    stub_request(:post, "#{host}/api/v1/app/app_id/endpoint/endpoint_id/replay-missing")
      .to_return(
        status: 200,
        body: ReplayOut_JSON
      )

    svx.endpoint.replay_missing("app_id", "endpoint_id", Svix::ReplayIn.new(since: Time.at(1740602143)))

    expect(WebMock).to(
      have_requested(
        :post,
        "#{host}/api/v1/app/app_id/endpoint/endpoint_id/replay-missing"
      )
        .with(body: "{\"since\":\"2025-02-26T20:35:43+00:00\"}")
    )
  end

  it "test Date response body" do
    stub_request(:get, "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id")
      .to_return(
        status: 200,
        body: ListResponseMessageAttemptOut_JSON
      )

    res = svx.message_attempt.list_by_endpoint("app_id", "endpoint_id")
    expect(res.data[0].timestamp.to_i).to(be(1566656122))

    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id"
      )
    )
  end

  it "test listResponseOut deserializes correctly" do
    stub_request(:get, "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id")
      .to_return(
        status: 200,
        body: "{\"data\":[],\"iterator\":null,\"prevIterator\":null,\"done\":true}"
      )
    svx.message_attempt.list_by_endpoint("app_id", "endpoint_id")

    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id"
      )
    )
  end

  it "test enum as query param" do
    stub_request(:get, "#{host}/api/v1/app")
      .with(query: hash_including({}))
      .to_return(
        status: 200,
        body: ListResponseAppOut_JSON
      )
    svx.application.list({order: Svix::Ordering::ASCENDING})

    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/app?order=ascending"
      )
    )
  end

  it "test list as query param" do
    stub_request(:get, "#{host}/api/v1/app/app_id/msg")
      .with(query: hash_including({}))
      .to_return(
        status: 200,
        body: ListResponseMessageOut_JSON
      )

    svx.message.list("app_id", {event_types: ["val8", "val1", "val5"]})

    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/app/app_id/msg?event_types=val1%2Cval5%2Cval8"
      )
    )
  end

  it "test header param sent" do
    stub_request(:post, "#{host}/api/v1/app/app_id/msg")
      .to_return(
        status: 200,
        body: MessageOut_JSON
      )

    svx.message.create("app_id", Svix::MessageIn.new(event_type: "event.type"), {:"idempotency-key" => "random-key"})
    expect(WebMock).to(
      have_requested(
        :post,
        "#{host}/api/v1/app/app_id/msg"
      )
        .with(
          headers: {
            "idempotency-key" => "random-key"
          }
        )
    )
  end

  it "no body in response does not return anything" do
    stub_request(:delete, "#{host}/api/v1/app/app_id")
      .to_return(
        status: 204
      )

    res = svx.application.delete("app_id")

    expect(res).to(be(nil))

    expect(WebMock).to(
      have_requested(
        :delete,
        "#{host}/api/v1/app/app_id"
      )
    )
  end

  it "sub-resource works" do
    stub_request(:get, "#{host}/api/v1/operational-webhook/endpoint")
      .to_return(
        status: 200,
        body: ListResponseOperationalWebhookEndpointOut_JSON
      )

    svx.operational_webhook_endpoint.list

    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/operational-webhook/endpoint"
      )
    )
  end

  it "integer enum deserialization" do
    stub_request(:get, "#{host}/api/v1/app/app_id/attempt/endpoint/endpoint_id")
      .to_return(
        status: 200,
        body: ListResponseMessageAttemptOut_JSON
      )

    res = svx.message_attempt.list_by_endpoint("app_id", "endpoint_id")
    expect(res.data[0].trigger_type).to(be(0))
  end

  it "string enum de/serialization" do
    stub_request(:get, "#{host}/api/v1/background-task")
      .with(query: hash_including({}))
      .to_return(
        status: 200,
        body: ListResponseBackgroundTaskOut_JSON
      )

    res = svx.background_task.list({task: Svix::BackgroundTaskType::APPLICATION_PURGE_CONTENT})

    expect(res.data[0].status).to(eq("running"))
    expect(WebMock).to(
      have_requested(
        :get,
        "#{host}/api/v1/background-task?task=application.purge_content"
      )
    )
  end

  it "non-camelCase field name" do
    stub_request(:post, "#{host}/api/v1/event-type/import/openapi")
      .to_return(
        status: 200,
        body: EventTypeImportOpenApiOut_JSON
      )

    res = svx.event_type.import_openapi(Svix::EventTypeImportOpenApiIn.new(dry_run: true))

    expect(res.data.to_modify).to(be_a_kind_of(Array))
    expect(WebMock).to(
      have_requested(
        :post,
        "#{host}/api/v1/event-type/import/openapi"
      )
    )
  end

  it "patch request body" do
    stub_request(:patch, "#{host}/api/v1/app/app_id/endpoint/endpoint_id")
      .to_return(
        status: 200,
        body: EndpointOut_JSON
      )

    # nullable field is set to nil
    svx.endpoint.patch("app_id", "endpoint_id", Svix::EndpointPatch.new(channels: nil))
    expect(WebMock).to(
      have_requested(
        :patch,
        "#{host}/api/v1/app/app_id/endpoint/endpoint_id"
      )
        .with(body: "{\"channels\":null}")
    )
    # nullable field not set at all, and non-nullable field set to nil
    svx.endpoint.patch("app_id", "endpoint_id", Svix::EndpointPatch.new(description: nil))
    expect(WebMock).to(
      have_requested(
        :patch,
        "#{host}/api/v1/app/app_id/endpoint/endpoint_id"
      )
        .with(body: "{}")
    )
    # nullable field set
    svx.endpoint.patch("app_id", "endpoint_id", Svix::EndpointPatch.new(channels: ["ch1", "ch2"]))
    expect(WebMock).to(
      have_requested(
        :patch,
        "#{host}/api/v1/app/app_id/endpoint/endpoint_id"
      )
        .with(body: "{\"channels\":[\"ch1\",\"ch2\"]}")
    )
  end

  it "arbitrary json object body" do
    stub_request(:post, "#{host}/api/v1/app/app_id/msg")
      .to_return(
        status: 200,
        body: MessageOut_JSON
      )

    svx.message.create("app_id", Svix::MessageIn.new(payload: {"key" => "val"}))

    expect(WebMock).to(
      have_requested(
        :post,
        "#{host}/api/v1/app/app_id/msg"
      )
        .with(body: "{\"payload\":{\"key\":\"val\"}}")
    )
  end

  it "MessageAttemptOut without msg" do
    stub_request(:get, "#{host}/api/v1/app/app/attempt/endpoint/edp")
      .to_return(
        status: 200,
        body: ListResponseMessageAttemptOut_without_msg_JSON
      )
    # make sure we don't panic here
    svx.message_attempt.list_by_endpoint("app", "edp")
  end

  it "test retry for status => 500" do
  end
end
