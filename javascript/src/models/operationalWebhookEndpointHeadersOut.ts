// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OperationalWebhookEndpointHeadersOut {
  headers: { [key: string]: string };
  sensitive: string[];
}

export const OperationalWebhookEndpointHeadersOutSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointHeadersOut {
    return {
      headers: object["headers"],
      sensitive: object["sensitive"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointHeadersOut): any {
    return {
      headers: self.headers,
      sensitive: self.sensitive,
    };
  },
};
