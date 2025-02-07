// this file is @generated

export interface OperationalWebhookEndpointHeadersOut {
  headers: { [key: string]: string };
  sensitive: string[];
}

export namespace OperationalWebhookEndpointHeadersOutUtil {
  export function _fromJsonObject(object: any): OperationalWebhookEndpointHeadersOut {
    return {
      headers: object["headers"],
      sensitive: object["sensitive"],
    };
  }

  export function _toJsonObject(self: OperationalWebhookEndpointHeadersOut): any {
    return {
      headers: self.headers,
      sensitive: self.sensitive,
    };
  }
}
