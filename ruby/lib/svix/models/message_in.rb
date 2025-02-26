# This file is @generated
require "json"

module Svix
  class MessageIn
    attr_accessor :application
    attr_accessor :channels
    attr_accessor :event_id
    attr_accessor :event_type
    attr_accessor :payload
    attr_accessor :payload_retention_hours
    attr_accessor :payload_retention_period
    attr_accessor :tags
    attr_accessor :transformations_params

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
        'application': Svix::ApplicationIn.deserialize(attributes["application"]),
        'channels': attributes["channels"],
        'event_id': attributes["eventId"],
        'event_type': attributes["eventType"],
        'payload': attributes["payload"],
        'payload_retention_hours': attributes["payloadRetentionHours"],
        'payload_retention_period': attributes["payloadRetentionPeriod"],
        'tags': attributes["tags"],
        'transformations_params': attributes["transformationsParams"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["application"] = @application.serialize
      out["channels"] = @channels
      out["eventId"] = @event_id
      out["eventType"] = @event_type
      out["payload"] = @payload
      out["payloadRetentionHours"] = @payload_retention_hours
      out["payloadRetentionPeriod"] = @payload_retention_period
      out["tags"] = @tags
      out["transformationsParams"] = @transformations_params
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
