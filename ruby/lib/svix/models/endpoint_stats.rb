# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointStats
    attr_accessor :fail
    attr_accessor :pending
    attr_accessor :sending
    attr_accessor :success

    ALL_FIELD ||= ["fail", "pending", "sending", "success"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointStats` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointStats")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["fail"] = attributes["fail"]
      attrs["pending"] = attributes["pending"]
      attrs["sending"] = attributes["sending"]
      attrs["success"] = attributes["success"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["fail"] = Svix::serialize_primitive(@fail) if @fail
      out["pending"] = Svix::serialize_primitive(@pending) if @pending
      out["sending"] = Svix::serialize_primitive(@sending) if @sending
      out["success"] = Svix::serialize_primitive(@success) if @success
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
