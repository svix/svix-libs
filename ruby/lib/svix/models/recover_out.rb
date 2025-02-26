# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RecoverOut
    attr_accessor :id
    attr_accessor :status
    attr_accessor :task

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
        'id': attributes["id"],
        'status': Svix::BackgroundTaskStatus.deserialize(attributes["status"]),
        'task': Svix::BackgroundTaskType.deserialize(attributes["task"]),
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["id"] = @id
      out["status"] = @status.serialize
      out["task"] = @task.serialize
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
