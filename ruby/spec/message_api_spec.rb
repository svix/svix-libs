# frozen_string_literal: true

require_relative '../lib/svix/api/message.rb'
require_relative '../lib/svix'

RSpec.describe Svix::Message do
  let(:app_id) { "app_123" }
  let(:msg_id) { "msg_123" }
  let(:options) { {with_content: true} }
  subject { described_class.new(param_mock) }

  let(:param_mock) { double("MessageApiParam") }
  let(:api_instance_mock) { double("MessageApi") }
  let(:api_class_mock) { double("MessageApiClass") }

  describe "message_in_raw" do
    it "works without content-type" do
      payload = "{ \"x\": true }"
      msg_in = Svix.message_in_raw(event_type: "x", payload: payload)

      expect(msg_in.event_type).to eq("x")
      expect(msg_in.payload).to eq({})
      expect(msg_in.transformations_params[:rawPayload]).to eq(payload)
      expect(msg_in.transformations_params[:headers]).to eq(nil)
    end

    it "works with content-type" do
      payload = "Hello, World!"
      content_type = "text/plain"
      msg_in = Svix.message_in_raw(event_type: "x", payload: payload, content_type: content_type)

      expect(msg_in.event_type).to eq("x")
      expect(msg_in.payload).to eq({})
      expect(msg_in.transformations_params[:rawPayload]).to eq(payload)
      expect(msg_in.transformations_params[:headers][:'content-type']).to eq(content_type)
    end

    it "works with other transformations params" do
      payload = "Hello, World!"
      content_type = "text/plain"
      msg_in = Svix.message_in_raw(
        event_type: "x",
        payload: payload,
        content_type: content_type,
        transformations_params: {
          :foo => "bar",
          :headers => {
            :'x-custom' => "baz",
          },
        },
      )

      expect(msg_in.event_type).to eq("x")
      expect(msg_in.payload).to eq({})
      expect(msg_in.transformations_params[:foo]).to eq("bar")
      expect(msg_in.transformations_params[:rawPayload]).to eq(payload)
      expect(msg_in.transformations_params[:headers][:'content-type']).to eq(content_type)
      expect(msg_in.transformations_params[:headers][:'x-custom']).to eq("baz")
    end
  end

  describe "#get" do
    # it "passes it's parameters to the correct method" do
    #   # Assert that the correct method is called with the correct parameters
    #   expect(api_instance_mock).to receive(:exc).with(app_id, msg_id, options)
    #
    #   subject.get(app_id, msg_id, options)
    # end

    # context "without options" do
    #   it "defaults to an empty hash" do
    #     # Assert that the correct method is called with the correct parameters
    #     expect(api_instance_mock).to receive(:v1_message_get).with(app_id, msg_id, {})
    #
    #     subject.get(app_id, msg_id)
    #   end
    # end
  end
end
