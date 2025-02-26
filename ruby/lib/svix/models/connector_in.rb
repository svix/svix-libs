# This file is @generated
require "json"

module Svix
  class ConnectorIn
    attr_accessor :description
    attr_accessor :feature_flag
    attr_accessor :filter_types
    attr_accessor :instructions
    attr_accessor :instructions_link
    attr_accessor :kind
    attr_accessor :logo
    attr_accessor :name
    attr_accessor :transformation

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
        'description': attributes["description"],
        'feature_flag': attributes["featureFlag"],
        'filter_types': attributes["filterTypes"],
        'instructions': attributes["instructions"],
        'instructions_link': attributes["instructionsLink"],
        'kind': Svix::ConnectorKind.deserialize(attributes["kind"]),
        'logo': attributes["logo"],
        'name': attributes["name"],
        'transformation': attributes["transformation"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["description"] = @description
      out["featureFlag"] = @feature_flag
      out["filterTypes"] = @filter_types
      out["instructions"] = @instructions
      out["instructionsLink"] = @instructions_link
      out["kind"] = @kind.serialize
      out["logo"] = @logo
      out["name"] = @name
      out["transformation"] = @transformation
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
