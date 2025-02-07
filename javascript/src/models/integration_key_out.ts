// this file is @generated

export interface IntegrationKeyOut {
  key: string;
}

export namespace IntegrationKeyOutUtil {
  export function _fromJsonObject(object: any): IntegrationKeyOut {
    return {
      key: object["key"],
    };
  }

  export function _toJsonObject(self: IntegrationKeyOut): any {
    return {
      key: self.key,
    };
  }
}
