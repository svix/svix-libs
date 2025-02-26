# This file is @generated
module Svix
  class ConnectorKind
    CUSTOM = "Custom".freeze
    CUSTOMER_IO = "CustomerIO".freeze
    DISCORD = "Discord".freeze
    HUBSPOT = "Hubspot".freeze
    INNGEST = "Inngest".freeze
    SALESFORCE = "Salesforce".freeze
    SEGMENT = "Segment".freeze
    SLACK = "Slack".freeze
    TEAMS = "Teams".freeze
    TRIGGER_DEV = "TriggerDev".freeze
    WINDMILL = "Windmill".freeze
    ZAPIER = "Zapier".freeze

    def self.all_vars
      @all_vars ||= [CUSTOM, CUSTOMER_IO, DISCORD, HUBSPOT, INNGEST, SALESFORCE, SEGMENT, SLACK, TEAMS, TRIGGER_DEV, WINDMILL, ZAPIER].freeze
    end

    def initialize(value)
      unless value.is_a?(String)
        fail ArgumentError, "The input argument (value) must be a String in `Svix::ConnectorKind` new method"
      end
      @value = value
      return value if ConnectorKind.all_vars.include?(value)
      raise "Invalid ENUM value '#{value}' for class #ConnectorKind"
    end

    def self.deserialize(value)
      new value
    end

    def serialize
      @value
    end
  end
end
