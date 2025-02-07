// this file is @generated

export interface IntegrationOut {
  createdAt: Date | null;
  /** The set of feature flags the integration has access to. */
  featureFlags?: string[];
  /** The integ's ID */
  id: string;
  name: string;
  updatedAt: Date | null;
}

export namespace IntegrationOutUtil {
  export function _fromJsonObject(object: any): IntegrationOut {
    return {
      createdAt: new Date(object["createdAt"]),
      featureFlags: object["featureFlags"],
      id: object["id"],
      name: object["name"],
      updatedAt: new Date(object["updatedAt"]),
    };
  }

  export function _toJsonObject(self: IntegrationOut): any {
    return {
      createdAt: self.createdAt,
      featureFlags: self.featureFlags,
      id: self.id,
      name: self.name,
      updatedAt: self.updatedAt,
    };
  }
}
