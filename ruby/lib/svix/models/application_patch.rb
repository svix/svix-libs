# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApplicationPatch
    attr_accessor :metadata
    attr_accessor :name
    attr_accessor :rate_limit
    attr_accessor :uid

    ALL_FIELD ||= ["metadata", "name", "rate_limit", "uid"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ApplicationPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ApplicationPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["metadata"] = attributes["metadata"]
      attrs["name"] = attributes["name"]
      attrs["rate_limit"] = attributes["rateLimit"]
      attrs["uid"] = attributes["uid"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["rateLimit"] = Svix::serialize_primitive(@rate_limit) if @__rate_limit_is_defined
      out["uid"] = Svix::serialize_primitive(@uid) if @__uid_is_defined
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
