# frozen_string_literal: true
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

    ALL_FIELD ||= [
      "description",
      "feature_flag",
      "filter_types",
      "instructions",
      "instructions_link",
      "kind",
      "logo",
      "name",
      "transformation"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ConnectorIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ConnectorIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["description"] = attributes["description"]
      attrs["feature_flag"] = attributes["featureFlag"]
      attrs["filter_types"] = attributes["filterTypes"]
      attrs["instructions"] = attributes["instructions"]
      attrs["instructions_link"] = attributes["instructionsLink"]
      attrs["kind"] = Svix::ConnectorKind.deserialize(attributes["kind"]) if attributes["kind"]
      attrs["logo"] = attributes["logo"]
      attrs["name"] = attributes["name"]
      attrs["transformation"] = attributes["transformation"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["featureFlag"] = Svix::serialize_primitive(@feature_flag) if @feature_flag
      out["filterTypes"] = Svix::serialize_primitive(@filter_types) if @filter_types
      out["instructions"] = Svix::serialize_primitive(@instructions) if @instructions
      out["instructionsLink"] = Svix::serialize_primitive(@instructions_link) if @instructions_link
      out["kind"] = @kind.serialize if @kind
      out["logo"] = Svix::serialize_primitive(@logo) if @logo
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["transformation"] = Svix::serialize_primitive(@transformation) if @transformation
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
