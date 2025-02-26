# This file is @generated
require "json"

module Svix
  class RecoverIn
    attr_accessor :since
    attr_accessor :until

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
        'since': attributes["since"],
        'until': attributes["until"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["since"] = @since
      out["until"] = @until
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
