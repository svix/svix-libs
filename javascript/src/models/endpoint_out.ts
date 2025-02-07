// this file is @generated

export interface EndpointOut {
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  createdAt: Date | null;
  /** An example endpoint name. */
  description: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  /** The ep's ID */
  id: string;
  metadata: { [key: string]: string };
  rateLimit?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  updatedAt: Date | null;
  url: string;
  version: number;
}

export namespace EndpointOutUtil {
  export function _fromJsonObject(object: any): EndpointOut {
    return {
      channels: object["channels"],
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
      version: object["version"],
    };
  }

  export function _toJsonObject(self: EndpointOut): any {
    return {
      channels: self.channels,
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
      version: self.version,
    };
  }
}
