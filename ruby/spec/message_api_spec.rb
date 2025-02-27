# frozen_string_literal: true
require "svix"
require_relative "../lib/svix/api/message.rb"

RSpec.describe Svix::Message do
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
end
