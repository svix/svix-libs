// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OperationalWebhookEndpointIn {
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
  rateLimit?: number | null;
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  secret?: string | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
}

export const OperationalWebhookEndpointInSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointIn {
    return {
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      secret: object["secret"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointIn): any {
    return {
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      secret: self.secret,
      uid: self.uid,
      url: self.url,
    };
  },
};
