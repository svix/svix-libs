// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface RecoverIn {
  since: Date | null;
  until?: Date | null | null;
}

export const RecoverInSerializer = {
  _fromJsonObject(object: any): RecoverIn {
    return {
      since: new Date(object["since"]),
      until: new Date(object["until"]),
    };
  },

  _toJsonObject(self: RecoverIn): any {
    return {
      since: self.since,
      until: self.until,
    };
  },
};
