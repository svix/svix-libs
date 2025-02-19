// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OperationalWebhookEndpointSecretOut {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key: string;
}

export const OperationalWebhookEndpointSecretOutSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointSecretOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointSecretOut): any {
    return {
      key: self.key,
    };
  },
};
