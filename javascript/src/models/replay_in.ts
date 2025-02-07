// this file is @generated

export interface ReplayIn {
  since: Date | null;
  until?: Date | null | null;
}

export namespace ReplayInUtil {
  export function _fromJsonObject(object: any): ReplayIn {
    return {
      since: new Date(object["since"]),
      until: new Date(object["until"]),
    };
  }

  export function _toJsonObject(self: ReplayIn): any {
    return {
      since: self.since,
      until: self.until,
    };
  }
}
