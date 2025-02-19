// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ReplayIn {
  since: Date | null;
  until?: Date | null | null;
}

export const ReplayInSerializer = {
  _fromJsonObject(object: any): ReplayIn {
    return {
      since: new Date(object["since"]),
      until: new Date(object["until"]),
    };
  },

  _toJsonObject(self: ReplayIn): any {
    return {
      since: self.since,
      until: self.until,
    };
  },
};
