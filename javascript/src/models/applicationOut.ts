// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApplicationOut {
  createdAt: Date | null;
  /** The Application's ID. */
  id: string;
  metadata: { [key: string]: string };
  name: string;
  rateLimit?: number | null;
  /** The Application's UID. */
  uid?: string | null;
  updatedAt: Date | null;
}

export const ApplicationOutSerializer = {
  _fromJsonObject(object: any): ApplicationOut {
    return {
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: ApplicationOut): any {
    return {
      createdAt: self.createdAt,
      id: self.id,
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
