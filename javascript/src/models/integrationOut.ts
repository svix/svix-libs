// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IntegrationOut {
  createdAt: Date | null;
  /** The set of feature flags the integration has access to. */
  featureFlags?: string[];
  /** The Integration's ID. */
  id: string;
  name: string;
  updatedAt: Date | null;
}

export const IntegrationOutSerializer = {
  _fromJsonObject(object: any): IntegrationOut {
    return {
      createdAt: new Date(object["createdAt"]),
      featureFlags: object["featureFlags"],
      id: object["id"],
      name: object["name"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: IntegrationOut): any {
    return {
      createdAt: self.createdAt,
      featureFlags: self.featureFlags,
      id: self.id,
      name: self.name,
      updatedAt: self.updatedAt,
    };
  },
};
