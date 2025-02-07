// this file is @generated

export interface DashboardAccessOut {
  token: string;
  url: string;
}

export namespace DashboardAccessOutUtil {
  export function _fromJsonObject(object: any): DashboardAccessOut {
    return {
      token: object["token"],
      url: object["url"],
    };
  }

  export function _toJsonObject(self: DashboardAccessOut): any {
    return {
      token: self.token,
      url: self.url,
    };
  }
}
