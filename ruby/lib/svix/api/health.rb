# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Health
    def initialize(client)
      @client = client
    end

    def get
      path = "/api/v1/health"
      @client.execute_request(
        "GET",
        path
      )
    end

  end
end
