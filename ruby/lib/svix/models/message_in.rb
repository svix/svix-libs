# frozen_string_literal: true
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

    ALL_FIELD ||= [
      "application",
      "channels",
      "event_id",
      "event_type",
      "payload",
      "payload_retention_hours",
      "payload_retention_period",
      "tags",
      "transformations_params"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessageIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["application"] = Svix::ApplicationIn.deserialize(attributes["application"]) if attributes["application"]
      attrs["channels"] = attributes["channels"]
      attrs["event_id"] = attributes["eventId"]
      attrs["event_type"] = attributes["eventType"]
      attrs["payload"] = attributes["payload"]
      attrs["payload_retention_hours"] = attributes["payloadRetentionHours"]
      attrs["payload_retention_period"] = attributes["payloadRetentionPeriod"]
      attrs["tags"] = attributes["tags"]
      attrs["transformations_params"] = attributes["transformationsParams"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["application"] = @application.serialize if @application
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["eventId"] = Svix::serialize_primitive(@event_id) if @event_id
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["payload"] = Svix::serialize_primitive(@payload) if @payload
      out["payloadRetentionHours"] = Svix::serialize_primitive(@payload_retention_hours) if @payload_retention_hours
      out["payloadRetentionPeriod"] = Svix::serialize_primitive(@payload_retention_period) if @payload_retention_period
      out["tags"] = Svix::serialize_primitive(@tags) if @tags
      out["transformationsParams"] = Svix::serialize_primitive(@transformations_params) if @transformations_params
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
