# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AppUsageStatsOut
    attr_accessor :id
    attr_accessor :status
    attr_accessor :task
    attr_accessor :unresolved_app_ids

    ALL_FIELD ||= ["id", "status", "task", "unresolved_app_ids"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::AppUsageStatsOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AppUsageStatsOut")
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
      attrs["unresolved_app_ids"] = attributes["unresolvedAppIds"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["status"] = @status.serialize if @status
      out["task"] = @task.serialize if @task
      out["unresolvedAppIds"] = Svix::serialize_primitive(@unresolved_app_ids) if @unresolved_app_ids
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
