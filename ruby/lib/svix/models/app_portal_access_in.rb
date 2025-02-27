# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AppPortalAccessIn
    attr_accessor :application
    attr_accessor :expiry
    attr_accessor :feature_flags
    attr_accessor :read_only

    ALL_FIELD ||= ["application", "expiry", "feature_flags", "read_only"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::AppPortalAccessIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AppPortalAccessIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["application"] = Svix::ApplicationIn.deserialize(attributes["application"]) if attributes["application"]
      attrs["expiry"] = attributes["expiry"]
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["read_only"] = attributes["readOnly"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["application"] = @application.serialize if @application
      out["expiry"] = Svix::serialize_primitive(@expiry) if @expiry
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["readOnly"] = Svix::serialize_primitive(@read_only) if @read_only
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
