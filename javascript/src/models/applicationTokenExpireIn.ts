// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApplicationTokenExpireIn {
  /** How many seconds until the old key is expired. */
  expiry?: number | null;
}

export const ApplicationTokenExpireInSerializer = {
  _fromJsonObject(object: any): ApplicationTokenExpireIn {
    return {
      expiry: object["expiry"],
    };
  },

  _toJsonObject(self: ApplicationTokenExpireIn): any {
    return {
      expiry: self.expiry,
    };
  },
};
