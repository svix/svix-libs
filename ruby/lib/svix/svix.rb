# frozen_string_literal: true

module Svix
  class SvixOptions
    attr_accessor :debug
    attr_accessor :server_url

    def initialize(debug = false, server_url = nil)
      @debug = debug
      @server_url = server_url
    end
  end

  class Client
    attr_accessor :application
    attr_accessor :authentication
    attr_accessor :background_task
    attr_accessor :endpoint
    attr_accessor :environment
    attr_accessor :event_type
    attr_accessor :health
    attr_accessor :integration
    attr_accessor :message
    attr_accessor :message_attempt
    attr_accessor :operational_webhook_endpoint
    attr_accessor :statistics

    def initialize(auth_token, options = SvixOptions.new)
      region = auth_token.split(".").last
      if region == "us"
        regional_url = "https://api.us.svix.com"
      elsif region == "eu"
        regional_url = "https://api.eu.svix.com"
      elsif region == "in"
        regional_url = "https://api.in.svix.com"
      else
        regional_url = "https://api.svix.com"
      end

      uri = URI(options.server_url || regional_url)
      api_client = SvixHttpClient.new(auth_token, uri, options.debug)

      @application = Application.new(api_client)
      @authentication = Authentication.new(api_client)
      @background_task = BackgroundTask.new(api_client)
      @endpoint = Endpoint.new(api_client)
      @environment = Environment.new(api_client)
      @event_type = EventType.new(api_client)
      @health = Health.new(api_client)
      @integration = Integration.new(api_client)
      @message = Message.new(api_client)
      @message_attempt = MessageAttempt.new(api_client)
      @operational_webhook_endpoint = OperationalWebhookEndpoint.new(api_client)
      @statistics = Statistics.new(api_client)
    end
  end
end
