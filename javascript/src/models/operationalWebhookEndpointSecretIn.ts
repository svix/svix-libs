// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OperationalWebhookEndpointSecretIn {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
}

export const OperationalWebhookEndpointSecretInSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointSecretIn {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointSecretIn): any {
    return {
      key: self.key,
    };
  },
};
