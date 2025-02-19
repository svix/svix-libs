// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointHeadersPatchIn {
  headers: { [key: string]: string };
}

export const EndpointHeadersPatchInSerializer = {
  _fromJsonObject(object: any): EndpointHeadersPatchIn {
    return {
      headers: object["headers"],
    };
  },

  _toJsonObject(self: EndpointHeadersPatchIn): any {
    return {
      headers: self.headers,
    };
  },
};
