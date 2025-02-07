// this file is @generated
/**
 * The value of the headers is returned in the `headers` field.
 *
 * Sensitive headers that have been redacted are returned in the sensitive field.
 */
export interface EndpointHeadersOut {
  headers: { [key: string]: string };
  sensitive: string[];
}

export namespace EndpointHeadersOutUtil {
  export function _fromJsonObject(object: any): EndpointHeadersOut {
    return {
      headers: object["headers"],
      sensitive: object["sensitive"],
    };
  }

  export function _toJsonObject(self: EndpointHeadersOut): any {
    return {
      headers: self.headers,
      sensitive: self.sensitive,
    };
  }
}
