# This file is @generated
require "json"

module Svix
  class AppPortalAccessIn
    attr_accessor :application
    attr_accessor :expiry
    attr_accessor :feature_flags
    attr_accessor :read_only

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
        'expiry': attributes["expiry"],
        'feature_flags': attributes["featureFlags"],
        'read_only': attributes["readOnly"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["application"] = @application.serialize
      out["expiry"] = @expiry
      out["featureFlags"] = @feature_flags
      out["readOnly"] = @read_only
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
