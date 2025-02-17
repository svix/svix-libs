// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
/**
 * The value of the headers is returned in the `headers` field.
 *
 * Sensitive headers that have been redacted are returned in the sensitive field.
 */
export interface EndpointHeadersOut {
  headers: { [key: string]: string };
  sensitive: string[];
}

export const EndpointHeadersOutSerializer = {
  _fromJsonObject(object: any): EndpointHeadersOut {
    return {
      headers: object["headers"],
      sensitive: object["sensitive"],
    };
  },

  _toJsonObject(self: EndpointHeadersOut): any {
    return {
      headers: self.headers,
      sensitive: self.sensitive,
    };
  },
};
