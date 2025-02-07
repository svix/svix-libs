// this file is @generated
/** Defines the ordering in a listing of results. */
export enum Ordering {
  Ascending = "ascending",
  Descending = "descending",
}

export namespace OrderingUtil {
  export function _fromJsonObject(object: any): Ordering {
    return object;
  }

  export function _toJsonObject(self: Ordering): any {
    return self;
  }
}
