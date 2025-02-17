// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointUpdate {
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
  rateLimit?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
  version?: number | null;
}

export const EndpointUpdateSerializer = {
  _fromJsonObject(object: any): EndpointUpdate {
    return {
      channels: object["channels"],
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      url: object["url"],
      version: object["version"],
    };
  },

  _toJsonObject(self: EndpointUpdate): any {
    return {
      channels: self.channels,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      uid: self.uid,
      url: self.url,
      version: self.version,
    };
  },
};
