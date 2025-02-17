// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventTypeOut {
  archived?: boolean;
  createdAt: Date | null;
  deprecated: boolean;
  description: string;
  featureFlag?: string | null;
  /** The event type group's name */
  groupName?: string | null;
  /** The event type's name */
  name: string;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
  updatedAt: Date | null;
}

export const EventTypeOutSerializer = {
  _fromJsonObject(object: any): EventTypeOut {
    return {
      archived: object["archived"],
      createdAt: new Date(object["createdAt"]),
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      groupName: object["groupName"],
      name: object["name"],
      schemas: object["schemas"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: EventTypeOut): any {
    return {
      archived: self.archived,
      createdAt: self.createdAt,
      deprecated: self.deprecated,
      description: self.description,
      featureFlag: self.featureFlag,
      groupName: self.groupName,
      name: self.name,
      schemas: self.schemas,
      updatedAt: self.updatedAt,
    };
  },
};
