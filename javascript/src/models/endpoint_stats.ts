// this file is @generated

export interface EndpointStats {
  fail: number;
  pending: number;
  sending: number;
  success: number;
}

export namespace EndpointStatsUtil {
  export function _fromJsonObject(object: any): EndpointStats {
    return {
      fail: object["fail"],
      pending: object["pending"],
      sending: object["sending"],
      success: object["success"],
    };
  }

  export function _toJsonObject(self: EndpointStats): any {
    return {
      fail: self.fail,
      pending: self.pending,
      sending: self.sending,
      success: self.success,
    };
  }
}
