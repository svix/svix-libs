# This file is @generated
module Svix
  # The sending status of the message:
  # - Success = 0
  # - Pending = 1
  # - Fail = 2
  # - Sending = 3
  class MessageStatus
    SUCCESS = 0.freeze
    PENDING = 1.freeze
    FAIL = 2.freeze
    SENDING = 3.freeze

    def self.all_vars
      @all_vars ||= [SUCCESS, PENDING, FAIL, SENDING].freeze
    end

    def initialize(value)
      unless value.is_a?(Integer)
        fail ArgumentError, "The input argument (value) must be a Integer in `Svix::MessageStatus` new method"
      end
      @value = value
      return value if MessageStatus.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #MessageStatus"
    end

    def self.deserialize(value)
      new value
    end

    def serialize
      @value
    end
  end
end
