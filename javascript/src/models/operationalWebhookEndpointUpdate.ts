// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OperationalWebhookEndpointUpdate {
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
  rateLimit?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
}

export const OperationalWebhookEndpointUpdateSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointUpdate {
    return {
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointUpdate): any {
    return {
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      uid: self.uid,
      url: self.url,
    };
  },
};
