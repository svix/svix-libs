# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Integration
    def initialize(client)
      @client = client
    end

    def list(app_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/integration"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseIntegrationOut.deserialize(res)
    end

    def create(app_id, integration_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/integration"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: integration_in
      )
      IntegrationOut.deserialize(res)
    end

    def get(app_id, integ_id)
      path = "/api/v1/app/#{app_id}/integration/#{integ_id}"
      res = @client.execute_request(
        "GET",
        path
      )
      IntegrationOut.deserialize(res)
    end

    def update(app_id, integ_id, integration_update)
      path = "/api/v1/app/#{app_id}/integration/#{integ_id}"
      res = @client.execute_request(
        "PUT",
        path,
        body: integration_update
      )
      IntegrationOut.deserialize(res)
    end

    def delete(app_id, integ_id)
      path = "/api/v1/app/#{app_id}/integration/#{integ_id}"
      @client.execute_request(
        "DELETE",
        path
      )
    end

    def get_key(app_id, integ_id)
      path = "/api/v1/app/#{app_id}/integration/#{integ_id}/key"
      res = @client.execute_request(
        "GET",
        path
      )
      IntegrationKeyOut.deserialize(res)
    end

    def rotate_key(app_id, integ_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/integration/#{integ_id}/key/rotate"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
      IntegrationKeyOut.deserialize(res)
    end

  end
end
