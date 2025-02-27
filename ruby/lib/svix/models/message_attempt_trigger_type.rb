# frozen_string_literal: true
# This file is @generated
module Svix
  # The reason an attempt was made:
  # - Scheduled = 0
  # - Manual = 1
  class MessageAttemptTriggerType
    SCHEDULED = 0.freeze
    MANUAL = 1.freeze

    def self.all_vars
      @all_vars ||= [SCHEDULED, MANUAL].freeze
    end

    def initialize(value)
      unless MessageAttemptTriggerType.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #MessageAttemptTriggerType"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if MessageAttemptTriggerType.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #MessageAttemptTriggerType"
    end

    def serialize
      @value
    end
  end
end
