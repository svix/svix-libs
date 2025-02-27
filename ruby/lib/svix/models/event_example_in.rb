# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventExampleIn
    attr_accessor :event_type
    attr_accessor :example_index

    ALL_FIELD ||= ["event_type", "example_index"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventExampleIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventExampleIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["event_type"] = attributes["eventType"]
      attrs["example_index"] = attributes["exampleIndex"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["exampleIndex"] = Svix::serialize_primitive(@example_index) if @example_index
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
