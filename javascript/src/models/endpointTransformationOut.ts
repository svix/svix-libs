// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointTransformationOut {
  code?: string | null;
  enabled?: boolean;
}

export const EndpointTransformationOutSerializer = {
  _fromJsonObject(object: any): EndpointTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: EndpointTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
