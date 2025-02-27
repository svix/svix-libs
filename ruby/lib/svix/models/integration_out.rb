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

    ALL_FIELD ||= ["created_at", "feature_flags", "id", "name", "updated_at"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::IntegrationOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IntegrationOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["id"] = attributes["id"]
      attrs["name"] = attributes["name"]
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
