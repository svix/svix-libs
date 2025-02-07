// this file is @generated

export interface OperationalWebhookEndpointHeadersIn {
  headers: { [key: string]: string };
}

export namespace OperationalWebhookEndpointHeadersInUtil {
  export function _fromJsonObject(object: any): OperationalWebhookEndpointHeadersIn {
    return {
      headers: object["headers"],
    };
  }

  export function _toJsonObject(self: OperationalWebhookEndpointHeadersIn): any {
    return {
      headers: self.headers,
    };
  }
}
