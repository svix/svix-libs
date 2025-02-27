# frozen_string_literal: true

module Svix
  class HTTPValidationError
    attr_accessor :detail

    def initialize(attributes = {})
      if (!attributes.is_a?(Hash))
        fail ArgumentError, "The input argument (attributes) must be a hash in `Svix::HTTPValidationError` initialize method"
      end
      @detail = attributes[:'detail']
    end
  end
end
