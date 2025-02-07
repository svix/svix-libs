// this file is @generated

export interface EventTypeUpdate {
  archived?: boolean;
  deprecated?: boolean;
  description: string;
  featureFlag?: string | null;
  /** The event type group's name */
  groupName?: string | null;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
}

export namespace EventTypeUpdateUtil {
  export function _fromJsonObject(object: any): EventTypeUpdate {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      groupName: object["groupName"],
      schemas: object["schemas"],
    };
  }

  export function _toJsonObject(self: EventTypeUpdate): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlag: self.featureFlag,
      groupName: self.groupName,
      schemas: self.schemas,
    };
  }
}
