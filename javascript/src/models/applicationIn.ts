// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApplicationIn {
  metadata?: { [key: string]: string };
  name: string;
  rateLimit?: number | null;
  /** Optional unique identifier for the application. */
  uid?: string | null;
}

export const ApplicationInSerializer = {
  _fromJsonObject(object: any): ApplicationIn {
    return {
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: ApplicationIn): any {
    return {
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      uid: self.uid,
    };
  },
};
