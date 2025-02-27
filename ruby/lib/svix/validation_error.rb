# frozen_string_literal: true

module Svix
  # Validation errors have their own schema to provide context for invalid requests eg. mismatched types and out of bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY error.
  class ValidationError
    # The location as a [`Vec`] of [`String`]s -- often in the form `[\"body\", \"field_name\"]`, `[\"query\", \"field_name\"]`, etc. They may, however, be arbitrarily deep.
    attr_accessor :loc

    # The message accompanying the validation error item.
    attr_accessor :msg

    # The type of error, often \"type_error\" or \"value_error\", but sometimes with more context like as \"value_error.number.not_ge\"
    attr_accessor :type

    def initialize(attributes = {})
      if (!attributes.is_a?(Hash))
        fail ArgumentError, "The input argument (attributes) must be a hash in `Svix::ValidationError` initialize method"
      end
      @loc = attributes[:'loc']
      @msg = attributes[:'msg']
      @type = attributes[:'type']
    end
  end
end
