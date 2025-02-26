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

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointPatch` new method"
      end
      attributes.each do |k, v|
        instance_variable_set "@#{k}", v
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = {
        'channels': attributes["channels"],
        'event_id': attributes["eventId"],
        'event_type': attributes["eventType"],
        'id': attributes["id"],
        'next_attempt': attributes["nextAttempt"],
        'payload': attributes["payload"],
        'status': Svix::MessageStatus.deserialize(attributes["status"]),
        'tags': attributes["tags"],
        'timestamp': attributes["timestamp"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["channels"] = @channels
      out["eventId"] = @event_id
      out["eventType"] = @event_type
      out["id"] = @id
      out["nextAttempt"] = @next_attempt
      out["payload"] = @payload
      out["status"] = @status.serialize
      out["tags"] = @tags
      out["timestamp"] = @timestamp
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
