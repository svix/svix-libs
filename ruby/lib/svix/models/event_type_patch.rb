# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypePatch
    attr_accessor :archived
    attr_accessor :deprecated
    attr_accessor :description
    attr_accessor :feature_flag
    attr_accessor :group_name
    attr_accessor :schemas

    ALL_FIELD ||= ["archived", "deprecated", "description", "feature_flag", "group_name", "schemas"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventTypePatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventTypePatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["archived"] = attributes["archived"]
      attrs["deprecated"] = attributes["deprecated"]
      attrs["description"] = attributes["description"]
      attrs["feature_flag"] = attributes["featureFlag"]
      attrs["group_name"] = attributes["groupName"]
      attrs["schemas"] = attributes["schemas"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["archived"] = Svix::serialize_primitive(@archived) if @archived
      out["deprecated"] = Svix::serialize_primitive(@deprecated) if @deprecated
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["featureFlag"] = Svix::serialize_primitive(@feature_flag) if @__feature_flag_is_defined
      out["groupName"] = Svix::serialize_primitive(@group_name) if @__group_name_is_defined
      out["schemas"] = Svix::serialize_primitive(@schemas) if @__schemas_is_defined
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
