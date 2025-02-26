# frozen_string_literal: true
# This file is @generated
module Svix
  class BackgroundTaskStatus
    RUNNING = "running".freeze
    FINISHED = "finished".freeze
    FAILED = "failed".freeze

    def self.all_vars
      @all_vars ||= [RUNNING, FINISHED, FAILED].freeze
    end

    def initialize(value)
      unless value.is_a?(String)
        fail ArgumentError, "The input argument (value) must be a String in `Svix::BackgroundTaskStatus` new method"
      end
      @value = value
      return value if BackgroundTaskStatus.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #BackgroundTaskStatus"
    end

    def self.deserialize(value)
      new value
    end

    def serialize
      @value
    end
  end
end
