// this file is @generated

export interface EndpointTransformationIn {
  code?: string | null;
  enabled?: boolean;
}

export namespace EndpointTransformationInUtil {
  export function _fromJsonObject(object: any): EndpointTransformationIn {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  }

  export function _toJsonObject(self: EndpointTransformationIn): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  }
}
