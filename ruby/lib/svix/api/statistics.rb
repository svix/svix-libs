# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Statistics
    def initialize(client)
      @client = client
    end

    def aggregate_app_stats(app_usage_stats_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/stats/usage/app"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: app_usage_stats_in
      )
      AppUsageStatsOut.deserialize(res)
    end

    def aggregate_event_types
      path = "/api/v1/stats/usage/event-types"
      res = @client.execute_request(
        "PUT",
        path
      )
      AggregateEventTypesOut.deserialize(res)
    end

  end
end
