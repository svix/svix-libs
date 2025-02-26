# This file is @generated
require "json"

module Svix
  class IntegrationIn
    attr_accessor :feature_flags
    attr_accessor :name

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
        'feature_flags': attributes["featureFlags"],
        'name': attributes["name"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["featureFlags"] = @feature_flags
      out["name"] = @name
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
