// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IntegrationIn {
  /** The set of feature flags the integration will have access to. */
  featureFlags?: string[];
  name: string;
}

export const IntegrationInSerializer = {
  _fromJsonObject(object: any): IntegrationIn {
    return {
      featureFlags: object["featureFlags"],
      name: object["name"],
    };
  },

  _toJsonObject(self: IntegrationIn): any {
    return {
      featureFlags: self.featureFlags,
      name: self.name,
    };
  },
};
