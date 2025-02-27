# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageAttemptOut
    attr_accessor :endpoint_id
    attr_accessor :id
    attr_accessor :msg
    attr_accessor :msg_id
    attr_accessor :response
    attr_accessor :response_duration_ms
    attr_accessor :response_status_code
    attr_accessor :status
    attr_accessor :timestamp
    attr_accessor :trigger_type
    attr_accessor :url

    ALL_FIELD ||= [
      "endpoint_id",
      "id",
      "msg",
      "msg_id",
      "response",
      "response_duration_ms",
      "response_status_code",
      "status",
      "timestamp",
      "trigger_type",
      "url"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessageAttemptOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageAttemptOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["endpoint_id"] = attributes["endpointId"]
      attrs["id"] = attributes["id"]
      attrs["msg"] = Svix::MessageOut.deserialize(attributes["msg"]) if attributes["msg"]
      attrs["msg_id"] = attributes["msgId"]
      attrs["response"] = attributes["response"]
      attrs["response_duration_ms"] = attributes["responseDurationMs"]
      attrs["response_status_code"] = attributes["responseStatusCode"]
      attrs["status"] = Svix::MessageStatus.deserialize(attributes["status"])
      attrs["timestamp"] = DateTime.rfc3339(attributes["timestamp"]).to_time
      attrs["trigger_type"] = Svix::MessageAttemptTriggerType.deserialize(attributes["triggerType"])
      attrs["url"] = attributes["url"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) if @endpoint_id
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["msg"] = @msg.serialize if @msg
      out["msgId"] = Svix::serialize_primitive(@msg_id) if @msg_id
      out["response"] = Svix::serialize_primitive(@response) if @response
      out["responseDurationMs"] = Svix::serialize_primitive(@response_duration_ms) if @response_duration_ms
      out["responseStatusCode"] = Svix::serialize_primitive(@response_status_code) if @response_status_code
      out["status"] = @status.serialize if @status
      out["timestamp"] = Svix::serialize_primitive(@timestamp) if @timestamp
      out["triggerType"] = @trigger_type.serialize if @trigger_type
      out["url"] = Svix::serialize_primitive(@url) if @url
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
