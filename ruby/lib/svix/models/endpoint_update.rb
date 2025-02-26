# This file is @generated
require "json"

module Svix
  class EndpointUpdate
    attr_accessor :channels
    attr_accessor :description
    attr_accessor :disabled
    attr_accessor :filter_types
    attr_accessor :metadata
    attr_accessor :rate_limit
    attr_accessor :uid
    attr_accessor :url
    attr_accessor :version

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
        'channels': attributes["channels"],
        'description': attributes["description"],
        'disabled': attributes["disabled"],
        'filter_types': attributes["filterTypes"],
        'metadata': attributes["metadata"],
        'rate_limit': attributes["rateLimit"],
        'uid': attributes["uid"],
        'url': attributes["url"],
        'version': attributes["version"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["channels"] = @channels
      out["description"] = @description
      out["disabled"] = @disabled
      out["filterTypes"] = @filter_types
      out["metadata"] = @metadata
      out["rateLimit"] = @rate_limit
      out["uid"] = @uid
      out["url"] = @url
      out["version"] = @version
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
