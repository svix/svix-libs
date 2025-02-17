// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointSecretRotateIn {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
}

export const EndpointSecretRotateInSerializer = {
  _fromJsonObject(object: any): EndpointSecretRotateIn {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: EndpointSecretRotateIn): any {
    return {
      key: self.key,
    };
  },
};
