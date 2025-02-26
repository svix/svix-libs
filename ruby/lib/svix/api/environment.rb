# This file is @generated
require "net/http"

module Svix
  class Environment
    def initialize(client)
      @client = client
    end

    def export(options = {})
      path = "/api/v1/environment/export"
      res = @client.execute_request(
        "POST",
        path,
        headers: { "idempotency-key" => options["idempotency-key"] },
      )
      EnvironmentOut.deserialize res
    end

    def import(environment_in, options = {})
      path = "/api/v1/environment/import"
      res = @client.execute_request(
        "POST",
        path,
        headers: { "idempotency-key" => options["idempotency-key"] },
        body: environment_in,
      )
    end
  end
end
