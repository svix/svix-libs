// this file is @generated

export interface RecoverIn {
  since: Date | null;
  until?: Date | null | null;
}

export namespace RecoverInUtil {
  export function _fromJsonObject(object: any): RecoverIn {
    return {
      since: new Date(object["since"]),
      until: new Date(object["until"]),
    };
  }

  export function _toJsonObject(self: RecoverIn): any {
    return {
      since: self.since,
      until: self.until,
    };
  }
}
