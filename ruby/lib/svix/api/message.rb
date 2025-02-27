# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  module_function
  # Creates a [`MessageIn`] with a raw string payload.
  #
  # The payload is not normalized on the server. Normally, payloads are required
  # to be JSON, and Svix will minify the payload before sending the webhook
  # (for example, by removing extraneous whitespace or unnecessarily escaped
  # characters in strings). With this function, the payload will be sent
  # "as is", without any minification or other processing.
  #
  # `attributes[:payload]` must be a string. An extra attribute `content_type`
  # is accepted that sets the `content-type` header of the webhook sent by Svix,
  # overwriting the default of `application/json` if specified. Other than that,
  # the attributes are forwarded [`MessageIn.new`], so all the same ones are
  # accepted.
  def message_in_raw(attributes = {})
    attributes[:transformations_params] ||= {}
    attributes[:transformations_params][:rawPayload] = attributes[:payload]
    attributes[:payload] = {}

    content_type = attributes.delete(:content_type)
    if content_type != nil
      attributes[:transformations_params][:headers] ||= {}
      attributes[:transformations_params][:headers][:"content-type"] = content_type
    end

    MessageIn.new(attributes)
  end

  class Message
    def initialize(client)
      @client = client
    end

    def list(app_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/msg"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "channel" => options["channel"],
          "before" => options["before"],
          "after" => options["after"],
          "with_content" => options["with_content"],
          "tag" => options["tag"],
          "event_types" => options["event_types"]
        }
      )
      ListResponseMessageOut.deserialize(res)
    end

    def create(app_id, message_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/msg"
      res = @client.execute_request(
        "POST",
        path,
        query_params: {
          "with_content" => options["with_content"]
        },
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: message_in
      )
      MessageOut.deserialize(res)
    end

    def expunge_all_contents(app_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/msg/expunge-all-contents"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
      ExpungAllContentsOut.deserialize(res)
    end

    def get(app_id, msg_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app/#{app_id}/msg/#{msg_id}"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "with_content" => options["with_content"]
        }
      )
      MessageOut.deserialize(res)
    end

    def expunge_content(app_id, msg_id)
      path = "/api/v1/app/#{app_id}/msg/#{msg_id}/content"
      @client.execute_request(
        "DELETE",
        path
      )
    end

  end
end
