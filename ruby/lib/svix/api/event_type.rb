# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EventType
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/event-type"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"],
          "include_archived" => options["include_archived"],
          "with_content" => options["with_content"]
        }
      )
      ListResponseEventTypeOut.deserialize(res)
    end

    def create(event_type_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/event-type"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: event_type_in
      )
      EventTypeOut.deserialize(res)
    end

    def import_openapi(event_type_import_open_api_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/event-type/import/openapi"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: event_type_import_open_api_in
      )
      EventTypeImportOpenApiOut.deserialize(res)
    end

    def get(event_type_name)
      path = "/api/v1/event-type/#{event_type_name}"
      res = @client.execute_request(
        "GET",
        path
      )
      EventTypeOut.deserialize(res)
    end

    def update(event_type_name, event_type_update)
      path = "/api/v1/event-type/#{event_type_name}"
      res = @client.execute_request(
        "PUT",
        path,
        body: event_type_update
      )
      EventTypeOut.deserialize(res)
    end

    def delete(event_type_name, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/event-type/#{event_type_name}"
      @client.execute_request(
        "DELETE",
        path,
        query_params: {
          "expunge" => options["expunge"]
        }
      )
    end

    def patch(event_type_name, event_type_patch)
      path = "/api/v1/event-type/#{event_type_name}"
      res = @client.execute_request(
        "PATCH",
        path,
        body: event_type_patch
      )
      EventTypeOut.deserialize(res)
    end

  end
end
