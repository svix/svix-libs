// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

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

export const EventTypeUpdateSerializer = {
  _fromJsonObject(object: any): EventTypeUpdate {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      groupName: object["groupName"],
      schemas: object["schemas"],
    };
  },

  _toJsonObject(self: EventTypeUpdate): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlag: self.featureFlag,
      groupName: self.groupName,
      schemas: self.schemas,
    };
  },
};
