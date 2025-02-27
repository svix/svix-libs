# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ListResponseEndpointMessageOut
    attr_accessor :data
    attr_accessor :done
    attr_accessor :iterator
    attr_accessor :prev_iterator

    ALL_FIELD ||= ["data", "done", "iterator", "prev_iterator"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::ListResponseEndpointMessageOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ListResponseEndpointMessageOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["data"] = attributes["data"].map { |v| Svix::EndpointMessageOut.deserialize(v) }
      attrs["done"] = attributes["done"]
      attrs["iterator"] = attributes["iterator"]
      attrs["prev_iterator"] = attributes["prevIterator"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["data"] = @data.map { |v| v.serialize } if @data
      out["done"] = Svix::serialize_primitive(@done) if @done
      out["iterator"] = Svix::serialize_primitive(@iterator) if @iterator
      out["prevIterator"] = Svix::serialize_primitive(@prev_iterator) if @prev_iterator
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
