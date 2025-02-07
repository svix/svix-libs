// this file is @generated

export interface EndpointHeadersPatchIn {
  headers: { [key: string]: string };
}

export namespace EndpointHeadersPatchInUtil {
  export function _fromJsonObject(object: any): EndpointHeadersPatchIn {
    return {
      headers: object["headers"],
    };
  }

  export function _toJsonObject(self: EndpointHeadersPatchIn): any {
    return {
      headers: self.headers,
    };
  }
}
