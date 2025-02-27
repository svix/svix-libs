# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApplicationOut
    attr_accessor :created_at
    attr_accessor :id
    attr_accessor :metadata
    attr_accessor :name
    attr_accessor :rate_limit
    attr_accessor :uid
    attr_accessor :updated_at

    ALL_FIELD ||= ["created_at", "id", "metadata", "name", "rate_limit", "uid", "updated_at"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ApplicationOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ApplicationOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["id"] = attributes["id"]
      attrs["metadata"] = attributes["metadata"]
      attrs["name"] = attributes["name"]
      attrs["rate_limit"] = attributes["rateLimit"]
      attrs["uid"] = attributes["uid"]
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["rateLimit"] = Svix::serialize_primitive(@rate_limit) if @rate_limit
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
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
