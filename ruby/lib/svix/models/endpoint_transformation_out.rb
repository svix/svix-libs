# This file is @generated
require "json"

module Svix
  class EndpointTransformationOut
    attr_accessor :code
    attr_accessor :enabled

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
        'code': attributes["code"],
        'enabled': attributes["enabled"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["code"] = @code
      out["enabled"] = @enabled
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
