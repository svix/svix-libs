// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface DashboardAccessOut {
  token: string;
  url: string;
}

export const DashboardAccessOutSerializer = {
  _fromJsonObject(object: any): DashboardAccessOut {
    return {
      token: object["token"],
      url: object["url"],
    };
  },

  _toJsonObject(self: DashboardAccessOut): any {
    return {
      token: self.token,
      url: self.url,
    };
  },
};
