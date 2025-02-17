// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventTypeIn {
  archived?: boolean;
  deprecated?: boolean;
  description: string;
  featureFlag?: string | null;
  /** The event type group's name */
  groupName?: string | null;
  /** The event type's name */
  name: string;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
}

export const EventTypeInSerializer = {
  _fromJsonObject(object: any): EventTypeIn {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      groupName: object["groupName"],
      name: object["name"],
      schemas: object["schemas"],
    };
  },

  _toJsonObject(self: EventTypeIn): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlag: self.featureFlag,
      groupName: self.groupName,
      name: self.name,
      schemas: self.schemas,
    };
  },
};
