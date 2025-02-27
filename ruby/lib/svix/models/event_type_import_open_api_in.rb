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

    ALL_FIELD ||= ["dry_run", "replace_all", "spec", "spec_raw"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EventTypeImportOpenApiIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventTypeImportOpenApiIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["dry_run"] = attributes["dryRun"]
      attrs["replace_all"] = attributes["replaceAll"]
      attrs["spec"] = attributes["spec"]
      attrs["spec_raw"] = attributes["specRaw"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["dryRun"] = Svix::serialize_primitive(@dry_run) if @dry_run
      out["replaceAll"] = Svix::serialize_primitive(@replace_all) if @replace_all
      out["spec"] = Svix::serialize_primitive(@spec) if @spec
      out["specRaw"] = Svix::serialize_primitive(@spec_raw) if @spec_raw
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
