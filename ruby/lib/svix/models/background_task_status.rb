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
      unless BackgroundTaskStatus.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #BackgroundTaskStatus"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if BackgroundTaskStatus.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #BackgroundTaskStatus"
    end

    def serialize
      @value
    end
  end
end
