# frozen_string_literal: true

module Svix
  class SvixError < StandardError
    attr_reader :message

    def initialize(message = nil)
      @message = message
    end
  end

  class WebhookVerificationError < SvixError
  end

  class WebhookSigningError < SvixError
  end
end
