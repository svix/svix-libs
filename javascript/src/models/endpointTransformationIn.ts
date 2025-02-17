// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointTransformationIn {
  code?: string | null;
  enabled?: boolean;
}

export const EndpointTransformationInSerializer = {
  _fromJsonObject(object: any): EndpointTransformationIn {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: EndpointTransformationIn): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
