# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointStats
    attr_accessor :fail
    attr_accessor :pending
    attr_accessor :sending
    attr_accessor :success

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
        'fail': attributes["fail"],
        'pending': attributes["pending"],
        'sending': attributes["sending"],
        'success': attributes["success"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["fail"] = @fail
      out["pending"] = @pending
      out["sending"] = @sending
      out["success"] = @success
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
