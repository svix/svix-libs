// this file is @generated

export interface OperationalWebhookEndpointSecretIn {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
}

export namespace OperationalWebhookEndpointSecretInUtil {
  export function _fromJsonObject(object: any): OperationalWebhookEndpointSecretIn {
    return {
      key: object["key"],
    };
  }

  export function _toJsonObject(self: OperationalWebhookEndpointSecretIn): any {
    return {
      key: self.key,
    };
  }
}
