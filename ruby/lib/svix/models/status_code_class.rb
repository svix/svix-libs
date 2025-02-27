# frozen_string_literal: true
# This file is @generated
module Svix
  # The different classes of HTTP status codes:
  # - CodeNone = 0
  # - Code1xx = 100
  # - Code2xx = 200
  # - Code3xx = 300
  # - Code4xx = 400
  # - Code5xx = 500
  class StatusCodeClass
    CODE_NONE = 0.freeze
    CODE1XX = 100.freeze
    CODE2XX = 200.freeze
    CODE3XX = 300.freeze
    CODE4XX = 400.freeze
    CODE5XX = 500.freeze

    def self.all_vars
      @all_vars ||= [CODE_NONE, CODE1XX, CODE2XX, CODE3XX, CODE4XX, CODE5XX].freeze
    end

    def initialize(value)
      unless StatusCodeClass.all_vars.include?(value)
        raise "Invalid ENUM value '#{value}' for class #StatusCodeClass"
      end

      @value = value
    end

    def self.deserialize(value)
      return value if StatusCodeClass.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #StatusCodeClass"
    end

    def serialize
      @value
    end
  end
end
