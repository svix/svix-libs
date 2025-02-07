// this file is @generated

export interface EndpointTransformationOut {
  code?: string | null;
  enabled?: boolean;
}

export namespace EndpointTransformationOutUtil {
  export function _fromJsonObject(object: any): EndpointTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  }

  export function _toJsonObject(self: EndpointTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  }
}
