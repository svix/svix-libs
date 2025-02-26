# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class IntegrationOut
    attr_accessor :created_at
    attr_accessor :feature_flags
    attr_accessor :id
    attr_accessor :name
    attr_accessor :updated_at

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
        'created_at': attributes["createdAt"],
        'feature_flags': attributes["featureFlags"],
        'id': attributes["id"],
        'name': attributes["name"],
        'updated_at': attributes["updatedAt"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["createdAt"] = @created_at
      out["featureFlags"] = @feature_flags
      out["id"] = @id
      out["name"] = @name
      out["updatedAt"] = @updated_at
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
