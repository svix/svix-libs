require "net/http"

module Svix
  class SvixHttpClient
    def initialize(token, base_url)
      @token = token
      @base_url = base_url
    end

    def execute_request(method, path, **kwargs)
      query_params = kwargs[:query_params] || {}
      headers = kwargs[:headers] || {}
      body = kwargs[:body] || {}

      uri = URI("#{@base_url}#{path}")
      encoded_query = encode_query_params(query_params)
      if encoded_query != ""
        uri.query = encoded_query
      end
      http = Net::HTTP.new(uri.host, uri.port)
      http.use_ssl = (uri.scheme == "https")

      # Dynamically select the request class based on method
      request_class = case method.to_s.upcase
        when "GET" then Net::HTTP::Get
        when "POST" then Net::HTTP::Post
        when "PUT" then Net::HTTP::Put
        when "DELETE" then Net::HTTP::Delete
        when "PATCH" then Net::HTTP::Patch
        when "HEAD" then Net::HTTP::Head
        else raise ArgumentError, "Unsupported HTTP method: #{method}"
        end

      # Create request object
      request = request_class.new(uri.request_uri)
      request["Authorization"] = "Bearer #{@token}"

      # Add headers
      headers.each { |key, value| request[key] = value }

      # Add body for non-GET requests
      if %w[POST PUT PATCH].include?(method.to_s.upcase) && !body.nil?
        request.body = body.to_json
        request["Content-Type"] = "application/json"
      end

      # Execute request
      debug_http_message(request, uri, "Request")
      res = http.request(request)
      res.header
      debug_http_message(res, uri, "Response")
      if Integer(res.code) == 204
        res
      elsif Integer(res.code) >= 200 && Integer(res.code) <= 299
        JSON.parse res.body
      else
        fail ApiError.new(:code => Integer(res.code), :response_headers => res.each_header.to_h, :response_body => res.body)
      end
    end

    private def encode_query_params(query_params = {})
      encoded_query_pairs = []
      query_params.each do |k, v|
        unless v.nil?
          if v.kind_of?(Array)
            encoded_query_pairs.append "#{k}=" + v.join(",")
          else
            encoded_query_pairs.append "#{k}=#{v}"
          end
        end
      end
      encoded_query_pairs.join("&")
    end

    def debug_http_message(message, uri, type)
      puts "\n=== HTTP/1.1 #{type} ==="
      case type
      when "Request"
        puts "#{message.method} #{uri.request_uri} HTTP/1.1"
        puts "Host: #{uri.host}"
        message.each_header { |name, value| puts "#{name}: #{value}" }
        puts "\n#{message.body}" if message.body
      when "Response"
        puts "HTTP/1.1 #{message.code} #{message.message}"
        message.each_header { |name, value| puts "#{name}: #{value}" }
        puts "\n#{message.body}" if message.body
      end
      puts "=== End #{type} ===\n"
    end
  end
end
