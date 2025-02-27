# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AggregateEventTypesOut
    attr_accessor :id
    attr_accessor :status
    attr_accessor :task

    ALL_FIELD ||= ["id", "status", "task"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::AggregateEventTypesOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AggregateEventTypesOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["id"] = attributes["id"]
      attrs["status"] = Svix::BackgroundTaskStatus.deserialize(attributes["status"])
      attrs["task"] = Svix::BackgroundTaskType.deserialize(attributes["task"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["status"] = @status.serialize if @status
      out["task"] = @task.serialize if @task
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
