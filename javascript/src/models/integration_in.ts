// this file is @generated

export interface IntegrationIn {
  /** The set of feature flags the integration will have access to. */
  featureFlags?: string[];
  name: string;
}

export namespace IntegrationInUtil {
  export function _fromJsonObject(object: any): IntegrationIn {
    return {
      featureFlags: object["featureFlags"],
      name: object["name"],
    };
  }

  export function _toJsonObject(self: IntegrationIn): any {
    return {
      featureFlags: self.featureFlags,
      name: self.name,
    };
  }
}
