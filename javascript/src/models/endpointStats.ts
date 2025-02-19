// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointStats {
  fail: number;
  pending: number;
  sending: number;
  success: number;
}

export const EndpointStatsSerializer = {
  _fromJsonObject(object: any): EndpointStats {
    return {
      fail: object["fail"],
      pending: object["pending"],
      sending: object["sending"],
      success: object["success"],
    };
  },

  _toJsonObject(self: EndpointStats): any {
    return {
      fail: self.fail,
      pending: self.pending,
      sending: self.sending,
      success: self.success,
    };
  },
};
