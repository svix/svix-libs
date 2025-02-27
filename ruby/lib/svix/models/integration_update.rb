# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class IntegrationUpdate
    attr_accessor :feature_flags
    attr_accessor :name

    ALL_FIELD ||= ["feature_flags", "name"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::IntegrationUpdate` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IntegrationUpdate")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["name"] = attributes["name"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["name"] = Svix::serialize_primitive(@name) if @name
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
