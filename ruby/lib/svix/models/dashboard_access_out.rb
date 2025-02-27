# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class DashboardAccessOut
    attr_accessor :token
    attr_accessor :url

    ALL_FIELD ||= ["token", "url"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::DashboardAccessOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::DashboardAccessOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["token"] = attributes["token"]
      attrs["url"] = attributes["url"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["token"] = Svix::serialize_primitive(@token) if @token
      out["url"] = Svix::serialize_primitive(@url) if @url
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
