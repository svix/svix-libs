# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypeImportOpenApiOutData
    attr_accessor :modified
    attr_accessor :to_modify

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
        'modified': attributes["modified"],
        'to_modify': attributes["to_modify"].map { |v| Svix::EventTypeFromOpenApi.deserialize v },
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["modified"] = @modified
      out["to_modify"] = @to_modify.map { |v| v.serialize }
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
