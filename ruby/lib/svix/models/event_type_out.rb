# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypeOut
    attr_accessor :archived
    attr_accessor :created_at
    attr_accessor :deprecated
    attr_accessor :description
    attr_accessor :feature_flag
    attr_accessor :group_name
    attr_accessor :name
    attr_accessor :schemas
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
        'archived': attributes["archived"],
        'created_at': attributes["createdAt"],
        'deprecated': attributes["deprecated"],
        'description': attributes["description"],
        'feature_flag': attributes["featureFlag"],
        'group_name': attributes["groupName"],
        'name': attributes["name"],
        'schemas': attributes["schemas"],
        'updated_at': attributes["updatedAt"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["archived"] = @archived
      out["createdAt"] = @created_at
      out["deprecated"] = @deprecated
      out["description"] = @description
      out["featureFlag"] = @feature_flag
      out["groupName"] = @group_name
      out["name"] = @name
      out["schemas"] = @schemas
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
