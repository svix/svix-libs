# This file is @generated
require "json"

module Svix
  class MessageAttemptOut
    attr_accessor :endpoint_id
    attr_accessor :id
    attr_accessor :msg
    attr_accessor :msg_id
    attr_accessor :response
    attr_accessor :response_duration_ms
    attr_accessor :response_status_code
    attr_accessor :status
    attr_accessor :timestamp
    attr_accessor :trigger_type
    attr_accessor :url

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointPatch` new method"
      end
      attributes.each do |k, v|
        instance_variable_set "@#{k}", v
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = {
        'endpoint_id': attributes["endpointId"],
        'id': attributes["id"],
        'msg': Svix::MessageOut.deserialize(attributes["msg"]),
        'msg_id': attributes["msgId"],
        'response': attributes["response"],
        'response_duration_ms': attributes["responseDurationMs"],
        'response_status_code': attributes["responseStatusCode"],
        'status': Svix::MessageStatus.deserialize(attributes["status"]),
        'timestamp': attributes["timestamp"],
        'trigger_type': Svix::MessageAttemptTriggerType.deserialize(attributes["triggerType"]),
        'url': attributes["url"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["endpointId"] = @endpoint_id
      out["id"] = @id
      out["msg"] = @msg.serialize
      out["msgId"] = @msg_id
      out["response"] = @response
      out["responseDurationMs"] = @response_duration_ms
      out["responseStatusCode"] = @response_status_code
      out["status"] = @status.serialize
      out["timestamp"] = @timestamp
      out["triggerType"] = @trigger_type.serialize
      out["url"] = @url
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
