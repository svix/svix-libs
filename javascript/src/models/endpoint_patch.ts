// this file is @generated

export interface EndpointPatch {
  channels?: string[] | null;
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
  /** The ep's UID */
  uid?: string | null;
  url?: string;
  version?: number;
}

export namespace EndpointPatchUtil {
  export function _fromJsonObject(object: any): EndpointPatch {
    return {
      channels: object["channels"],
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      secret: object["secret"],
      uid: object["uid"],
      url: object["url"],
      version: object["version"],
    };
  }

  export function _toJsonObject(self: EndpointPatch): any {
    return {
      channels: self.channels,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      secret: self.secret,
      uid: self.uid,
      url: self.url,
      version: self.version,
    };
  }
}
