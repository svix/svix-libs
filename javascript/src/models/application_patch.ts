// this file is @generated

export interface ApplicationPatch {
  metadata?: { [key: string]: string };
  name?: string;
  rateLimit?: number | null;
  /** The app's UID */
  uid?: string | null;
}

export namespace ApplicationPatchUtil {
  export function _fromJsonObject(object: any): ApplicationPatch {
    return {
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
    };
  }

  export function _toJsonObject(self: ApplicationPatch): any {
    return {
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      uid: self.uid,
    };
  }
}
