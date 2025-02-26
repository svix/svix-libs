# This file is @generated
require "json"

module Svix
  class OperationalWebhookEndpointIn
    attr_accessor :description
    attr_accessor :disabled
    attr_accessor :filter_types
    attr_accessor :metadata
    attr_accessor :rate_limit
    attr_accessor :secret
    attr_accessor :uid
    attr_accessor :url

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
        'description': attributes["description"],
        'disabled': attributes["disabled"],
        'filter_types': attributes["filterTypes"],
        'metadata': attributes["metadata"],
        'rate_limit': attributes["rateLimit"],
        'secret': attributes["secret"],
        'uid': attributes["uid"],
        'url': attributes["url"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["description"] = @description
      out["disabled"] = @disabled
      out["filterTypes"] = @filter_types
      out["metadata"] = @metadata
      out["rateLimit"] = @rate_limit
      out["secret"] = @secret
      out["uid"] = @uid
      out["url"] = @url
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
