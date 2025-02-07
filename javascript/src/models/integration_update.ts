// this file is @generated

export interface IntegrationUpdate {
  /** The set of feature flags the integration will have access to. */
  featureFlags?: string[];
  name: string;
}

export namespace IntegrationUpdateUtil {
  export function _fromJsonObject(object: any): IntegrationUpdate {
    return {
      featureFlags: object["featureFlags"],
      name: object["name"],
    };
  }

  export function _toJsonObject(self: IntegrationUpdate): any {
    return {
      featureFlags: self.featureFlags,
      name: self.name,
    };
  }
}
