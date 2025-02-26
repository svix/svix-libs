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
        'created_at': attributes["createdAt"],
        'id': attributes["id"],
        'metadata': attributes["metadata"],
        'name': attributes["name"],
        'rate_limit': attributes["rateLimit"],
        'uid': attributes["uid"],
        'updated_at': attributes["updatedAt"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["createdAt"] = @created_at
      out["id"] = @id
      out["metadata"] = @metadata
      out["name"] = @name
      out["rateLimit"] = @rate_limit
      out["uid"] = @uid
      out["updatedAt"] = @updated_at
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
