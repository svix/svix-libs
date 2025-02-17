// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IntegrationUpdate {
  /** The set of feature flags the integration will have access to. */
  featureFlags?: string[];
  name: string;
}

export const IntegrationUpdateSerializer = {
  _fromJsonObject(object: any): IntegrationUpdate {
    return {
      featureFlags: object["featureFlags"],
      name: object["name"],
    };
  },

  _toJsonObject(self: IntegrationUpdate): any {
    return {
      featureFlags: self.featureFlags,
      name: self.name,
    };
  },
};
