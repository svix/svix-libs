# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Application
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseApplicationOut.deserialize(res)
    end

    def create(application_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def get_or_create(application_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app"
      res = @client.execute_request(
        "POST",
        path,
        query_params: {
          "get_if_exists" => true
        },
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def get(app_id)
      path = "/api/v1/app/#{app_id}"
      res = @client.execute_request(
        "GET",
        path
      )
      ApplicationOut.deserialize(res)
    end

    def update(app_id, application_in)
      path = "/api/v1/app/#{app_id}"
      res = @client.execute_request(
        "PUT",
        path,
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def delete(app_id)
      path = "/api/v1/app/#{app_id}"
      @client.execute_request(
        "DELETE",
        path
      )
    end

    def patch(app_id, application_patch)
      path = "/api/v1/app/#{app_id}"
      res = @client.execute_request(
        "PATCH",
        path,
        body: application_patch
      )
      ApplicationOut.deserialize(res)
    end

  end
end
