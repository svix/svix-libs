// this file is @generated

export interface ApplicationIn {
  metadata?: { [key: string]: string };
  name: string;
  rateLimit?: number | null;
  /** Optional unique identifier for the application. */
  uid?: string | null;
}

export namespace ApplicationInUtil {
  export function _fromJsonObject(object: any): ApplicationIn {
    return {
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
    };
  }

  export function _toJsonObject(self: ApplicationIn): any {
    return {
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      uid: self.uid,
    };
  }
}
