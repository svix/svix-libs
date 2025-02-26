# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # The value of the headers is returned in the `headers` field.
  #
  # Sensitive headers that have been redacted are returned in the sensitive field.
  class EndpointHeadersOut
    attr_accessor :headers
    attr_accessor :sensitive

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
        'headers': attributes["headers"],
        'sensitive': attributes["sensitive"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["headers"] = @headers
      out["sensitive"] = @sensitive
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
