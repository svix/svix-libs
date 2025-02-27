# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # A model containing information on a given message plus additional fields on the last attempt for that message.
  class EndpointMessageOut
    attr_accessor :channels
    attr_accessor :event_id
    attr_accessor :event_type
    attr_accessor :id
    attr_accessor :next_attempt
    attr_accessor :payload
    attr_accessor :status
    attr_accessor :tags
    attr_accessor :timestamp

    ALL_FIELD ||= ["channels", "event_id", "event_type", "id", "next_attempt", "payload", "status", "tags", "timestamp"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointMessageOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointMessageOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["channels"] = attributes["channels"]
      attrs["event_id"] = attributes["eventId"]
      attrs["event_type"] = attributes["eventType"]
      attrs["id"] = attributes["id"]
      attrs["next_attempt"] = DateTime.rfc3339(attributes["nextAttempt"]).to_time if attributes["nextAttempt"]
      attrs["payload"] = attributes["payload"]
      attrs["status"] = Svix::MessageStatus.deserialize(attributes["status"])
      attrs["tags"] = attributes["tags"]
      attrs["timestamp"] = DateTime.rfc3339(attributes["timestamp"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["eventId"] = Svix::serialize_primitive(@event_id) if @event_id
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["nextAttempt"] = Svix::serialize_primitive(@next_attempt) if @next_attempt
      out["payload"] = Svix::serialize_primitive(@payload) if @payload
      out["status"] = @status.serialize if @status
      out["tags"] = Svix::serialize_primitive(@tags) if @tags
      out["timestamp"] = Svix::serialize_primitive(@timestamp) if @timestamp
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
