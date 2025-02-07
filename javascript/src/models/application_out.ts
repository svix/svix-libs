// this file is @generated

export interface ApplicationOut {
  createdAt: Date | null;
  /** The app's ID */
  id: string;
  metadata: { [key: string]: string };
  name: string;
  rateLimit?: number | null;
  /** The app's UID */
  uid?: string | null;
  updatedAt: Date | null;
}

export namespace ApplicationOutUtil {
  export function _fromJsonObject(object: any): ApplicationOut {
    return {
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  }

  export function _toJsonObject(self: ApplicationOut): any {
    return {
      createdAt: self.createdAt,
      id: self.id,
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  }
}
