# frozen_string_literal: true
# This file is @generated
module Svix
  class BackgroundTaskType
    ENDPOINT_REPLAY = "endpoint.replay".freeze
    ENDPOINT_RECOVER = "endpoint.recover".freeze
    APPLICATION_STATS = "application.stats".freeze
    MESSAGE_BROADCAST = "message.broadcast".freeze
    SDK_GENERATE = "sdk.generate".freeze
    EVENT_TYPE_AGGREGATE = "event-type.aggregate".freeze
    APPLICATION_PURGE_CONTENT = "application.purge_content".freeze

    def self.all_vars
      @all_vars ||= [
        ENDPOINT_REPLAY,
        ENDPOINT_RECOVER,
        APPLICATION_STATS,
        MESSAGE_BROADCAST,
        SDK_GENERATE,
        EVENT_TYPE_AGGREGATE,
        APPLICATION_PURGE_CONTENT
      ].freeze
    end

    def initialize(value)
      unless BackgroundTaskType.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #BackgroundTaskType"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if BackgroundTaskType.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #BackgroundTaskType"
    end

    def serialize
      @value
    end
  end
end
