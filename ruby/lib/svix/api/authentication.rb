# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Authentication
    def initialize(client)
      @client = client
    end

    def app_portal_access(app_id, app_portal_access_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/auth/app-portal-access/#{app_id}"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: app_portal_access_in
      )
      AppPortalAccessOut.deserialize(res)
    end

    def expire_all(app_id, application_token_expire_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/auth/app/#{app_id}/expire-all"
      @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: application_token_expire_in
      )
    end

    def dashboard_access(app_id, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/auth/dashboard-access/#{app_id}"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
      DashboardAccessOut.deserialize(res)
    end

    def logout(options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/auth/logout"
      @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
    end

  end
end
