// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface AppPortalAccessOut {
  token: string;
  url: string;
}

export const AppPortalAccessOutSerializer = {
  _fromJsonObject(object: any): AppPortalAccessOut {
    return {
      token: object["token"],
      url: object["url"],
    };
  },

  _toJsonObject(self: AppPortalAccessOut): any {
    return {
      token: self.token,
      url: self.url,
    };
  },
};
