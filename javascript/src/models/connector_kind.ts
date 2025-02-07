// this file is @generated

export enum ConnectorKind {
  Custom = "Custom",
  CustomerIo = "CustomerIO",
  Discord = "Discord",
  Hubspot = "Hubspot",
  Inngest = "Inngest",
  Salesforce = "Salesforce",
  Segment = "Segment",
  Slack = "Slack",
  Teams = "Teams",
  TriggerDev = "TriggerDev",
  Windmill = "Windmill",
  Zapier = "Zapier",
}

export namespace ConnectorKindUtil {
  export function _fromJsonObject(object: any): ConnectorKind {
    return object;
  }

  export function _toJsonObject(self: ConnectorKind): any {
    return self;
  }
}
