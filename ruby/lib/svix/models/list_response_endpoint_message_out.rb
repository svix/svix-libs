# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ListResponseEndpointMessageOut
    attr_accessor :data
    attr_accessor :done
    attr_accessor :iterator
    attr_accessor :prev_iterator

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
        'data': attributes["data"].map { |v| Svix::EndpointMessageOut.deserialize v },
        'done': attributes["done"],
        'iterator': attributes["iterator"],
        'prev_iterator': attributes["prevIterator"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["data"] = @data.map { |v| v.serialize }
      out["done"] = @done
      out["iterator"] = @iterator
      out["prevIterator"] = @prev_iterator
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
