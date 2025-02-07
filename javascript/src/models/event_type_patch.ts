// this file is @generated

export interface EventTypePatch {
  archived?: boolean;
  deprecated?: boolean;
  description?: string;
  featureFlag?: string | null;
  /** The event type group's name */
  groupName?: string | null;
  schemas?: any | null;
}

export namespace EventTypePatchUtil {
  export function _fromJsonObject(object: any): EventTypePatch {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      groupName: object["groupName"],
      schemas: object["schemas"],
    };
  }

  export function _toJsonObject(self: EventTypePatch): any {
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
