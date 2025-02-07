// this file is @generated

export interface ApplicationTokenExpireIn {
  /** How many seconds until the old key is expired. */
  expiry?: number | null;
}

export namespace ApplicationTokenExpireInUtil {
  export function _fromJsonObject(object: any): ApplicationTokenExpireIn {
    return {
      expiry: object["expiry"],
    };
  }

  export function _toJsonObject(self: ApplicationTokenExpireIn): any {
    return {
      expiry: self.expiry,
    };
  }
}
