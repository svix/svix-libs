// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OperationalWebhookEndpointOut {
  createdAt: Date | null;
  /** An example endpoint name. */
  description: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  /** The Endpoint's ID. */
  id: string;
  metadata: { [key: string]: string };
  rateLimit?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  updatedAt: Date | null;
  url: string;
}

export const OperationalWebhookEndpointOutSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      id: object["id"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
      url: object["url"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      id: self.id,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      uid: self.uid,
      updatedAt: self.updatedAt,
      url: self.url,
    };
  },
};
