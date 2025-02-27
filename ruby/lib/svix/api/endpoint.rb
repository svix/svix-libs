# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Endpoint
    def initialize(client)
      @client = client
    end

    def list(app_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseEndpointOut.deserialize(res)
    end

    def create(app_id, endpoint_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: endpoint_in
      )
      EndpointOut.deserialize(res)
    end

    def get(app_id, endpoint_id)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "GET",
        path
      )
      EndpointOut.deserialize(res)
    end

    def update(app_id, endpoint_id, endpoint_update)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "PUT",
        path,
        body: endpoint_update
      )
      EndpointOut.deserialize(res)
    end

    def delete(app_id, endpoint_id)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}"
      @client.execute_request(
        "DELETE",
        path
      )
    end

    def patch(app_id, endpoint_id, endpoint_patch)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "PATCH",
        path,
        body: endpoint_patch
      )
      EndpointOut.deserialize(res)
    end

    def get_headers(app_id, endpoint_id)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/headers"
      res = @client.execute_request(
        "GET",
        path
      )
      EndpointHeadersOut.deserialize(res)
    end

    def update_headers(app_id, endpoint_id, endpoint_headers_in)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/headers"
      @client.execute_request(
        "PUT",
        path,
        body: endpoint_headers_in
      )
    end

    def patch_headers(app_id, endpoint_id, endpoint_headers_patch_in)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/headers"
      @client.execute_request(
        "PATCH",
        path,
        body: endpoint_headers_patch_in
      )
    end

    def recover(app_id, endpoint_id, recover_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/recover"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: recover_in
      )
      RecoverOut.deserialize(res)
    end

    def replay_missing(app_id, endpoint_id, replay_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/replay-missing"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: replay_in
      )
      ReplayOut.deserialize(res)
    end

    def get_secret(app_id, endpoint_id)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/secret"
      res = @client.execute_request(
        "GET",
        path
      )
      EndpointSecretOut.deserialize(res)
    end

    def rotate_secret(app_id, endpoint_id, endpoint_secret_rotate_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/secret/rotate"
      @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: endpoint_secret_rotate_in
      )
    end

    def send_example(app_id, endpoint_id, event_example_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/send-example"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: event_example_in
      )
      MessageOut.deserialize(res)
    end

    def get_stats(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/stats"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "since" => options["since"],
          "until" => options["until"]
        }
      )
      EndpointStats.deserialize(res)
    end

    def transformation_get(app_id, endpoint_id)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation"
      res = @client.execute_request(
        "GET",
        path
      )
      EndpointTransformationOut.deserialize(res)
    end

    def transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in)
      path = "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation"
      @client.execute_request(
        "PATCH",
        path,
        body: endpoint_transformation_in
      )
    end

  end
end
