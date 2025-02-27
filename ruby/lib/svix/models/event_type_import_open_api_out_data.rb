# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypeImportOpenApiOutData
    attr_accessor :modified
    attr_accessor :to_modify

    ALL_FIELD ||= ["modified", "to_modify"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EventTypeImportOpenApiOutData` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventTypeImportOpenApiOutData")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["modified"] = attributes["modified"]
      if attributes["to_modify"]
        attrs["to_modify"] = attributes["to_modify"].map { |v| Svix::EventTypeFromOpenApi.deserialize(v) }
      end

      new(attrs)
    end

    def serialize
      out = Hash.new
      out["modified"] = Svix::serialize_primitive(@modified) if @modified
      out["to_modify"] = @to_modify.map { |v| v.serialize } if @to_modify
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
