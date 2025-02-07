// this file is @generated

export interface EndpointHeadersIn {
  headers: { [key: string]: string };
}

export namespace EndpointHeadersInUtil {
  export function _fromJsonObject(object: any): EndpointHeadersIn {
    return {
      headers: object["headers"],
    };
  }

  export function _toJsonObject(self: EndpointHeadersIn): any {
    return {
      headers: self.headers,
    };
  }
}
