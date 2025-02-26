# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Import a list of event types from webhooks defined in an OpenAPI spec.
  #
  # The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
  class EventTypeImportOpenApiIn
    attr_accessor :dry_run
    attr_accessor :replace_all
    attr_accessor :spec
    attr_accessor :spec_raw

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
        'dry_run': attributes["dryRun"],
        'replace_all': attributes["replaceAll"],
        'spec': attributes["spec"],
        'spec_raw': attributes["specRaw"],
      }
      new attrs
    end

    def serialize
      out = Hash.new
      out["dryRun"] = @dry_run
      out["replaceAll"] = @replace_all
      out["spec"] = @spec
      out["specRaw"] = @spec_raw
      out.compact
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
