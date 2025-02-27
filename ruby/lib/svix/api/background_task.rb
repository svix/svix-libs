# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class BackgroundTask
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/background-task"
      res = @client.execute_request(
        "GET",
        path,
        query_params: {
          "status" => options["status"],
          "task" => options["task"],
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseBackgroundTaskOut.deserialize(res)
    end

    def get(task_id)
      path = "/api/v1/background-task/#{task_id}"
      res = @client.execute_request(
        "GET",
        path
      )
      BackgroundTaskOut.deserialize(res)
    end

  end
end
