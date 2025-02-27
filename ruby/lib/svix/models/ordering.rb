# frozen_string_literal: true
# This file is @generated
module Svix
  # Defines the ordering in a listing of results.
  class Ordering
    ASCENDING = "ascending".freeze
    DESCENDING = "descending".freeze

    def self.all_vars
      @all_vars ||= [ASCENDING, DESCENDING].freeze
    end

    def initialize(value)
      unless Ordering.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #Ordering"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if Ordering.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #Ordering"
    end

    def serialize
      @value
    end
  end
end
