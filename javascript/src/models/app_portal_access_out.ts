// this file is @generated

export interface AppPortalAccessOut {
  token: string;
  url: string;
}

export namespace AppPortalAccessOutUtil {
  export function _fromJsonObject(object: any): AppPortalAccessOut {
    return {
      token: object["token"],
      url: object["url"],
    };
  }

  export function _toJsonObject(self: AppPortalAccessOut): any {
    return {
      token: self.token,
      url: self.url,
    };
  }
}
