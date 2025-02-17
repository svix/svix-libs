// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApplicationPatch {
  metadata?: { [key: string]: string };
  name?: string;
  rateLimit?: number | null;
  /** The Application's UID. */
  uid?: string | null;
}

export const ApplicationPatchSerializer = {
  _fromJsonObject(object: any): ApplicationPatch {
    return {
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: ApplicationPatch): any {
    return {
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      uid: self.uid,
    };
  },
};
