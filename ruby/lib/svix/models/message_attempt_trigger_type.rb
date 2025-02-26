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
      unless value.is_a?(Integer)
        fail ArgumentError, "The input argument (value) must be a Integer in `Svix::MessageAttemptTriggerType` new method"
      end
      @value = value
      return value if MessageAttemptTriggerType.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #MessageAttemptTriggerType"
    end

    def self.deserialize(value)
      new value
    end

    def serialize
      @value
    end
  end
end
