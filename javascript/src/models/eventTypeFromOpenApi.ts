// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventTypeFromOpenApi {
  deprecated: boolean;
  description: string;
  featureFlag?: string | null;
  /** The event type group's name */
  groupName?: string | null;
  /** The event type's name */
  name: string;
  schemas?: any | null;
}

export const EventTypeFromOpenApiSerializer = {
  _fromJsonObject(object: any): EventTypeFromOpenApi {
    return {
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      groupName: object["groupName"],
      name: object["name"],
      schemas: object["schemas"],
    };
  },

  _toJsonObject(self: EventTypeFromOpenApi): any {
    return {
      deprecated: self.deprecated,
      description: self.description,
      featureFlag: self.featureFlag,
      groupName: self.groupName,
      name: self.name,
      schemas: self.schemas,
    };
  },
};
