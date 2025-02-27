# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class OperationalWebhookEndpoint
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/operational-webhook/endpoint"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseOperationalWebhookEndpointOut.deserialize(res)
    end

    def create(operational_webhook_endpoint_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/operational-webhook/endpoint"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: operational_webhook_endpoint_in
      )
      OperationalWebhookEndpointOut.deserialize(res)
    end

    def get(endpoint_id)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "GET",
        path
      )
      OperationalWebhookEndpointOut.deserialize(res)
    end

    def update(endpoint_id, operational_webhook_endpoint_update)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "PUT",
        path,
        body: operational_webhook_endpoint_update
      )
      OperationalWebhookEndpointOut.deserialize(res)
    end

    def delete(endpoint_id)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}"
      @client.execute_request(
        "DELETE",
        path
      )
    end

    def get_headers(endpoint_id)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}/headers"
      res = @client.execute_request(
        "GET",
        path
      )
      OperationalWebhookEndpointHeadersOut.deserialize(res)
    end

    def update_headers(endpoint_id, operational_webhook_endpoint_headers_in)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}/headers"
      @client.execute_request(
        "PUT",
        path,
        body: operational_webhook_endpoint_headers_in
      )
    end

    def get_secret(endpoint_id)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}/secret"
      res = @client.execute_request(
        "GET",
        path
      )
      OperationalWebhookEndpointSecretOut.deserialize(res)
    end

    def rotate_secret(endpoint_id, operational_webhook_endpoint_secret_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/operational-webhook/endpoint/#{endpoint_id}/secret/rotate"
      @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: operational_webhook_endpoint_secret_in
      )
    end

  end
end
