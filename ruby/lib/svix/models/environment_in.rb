# This file is @generated
require "json"

module Svix
  class EnvironmentIn
    attr_accessor :connectors
    attr_accessor :event_types
    attr_accessor :settings

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
        'connectors': attributes["connectors"].map { |v| Svix::ConnectorIn.deserialize v },
        'event_types': attributes["eventTypes"].map { |v| Svix::EventTypeIn.deserialize v },
        'settings': attributes["settings"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["connectors"] = @connectors.map { |v| v.serialize }
      out["eventTypes"] = @event_types.map { |v| v.serialize }
      out["settings"] = @settings
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
