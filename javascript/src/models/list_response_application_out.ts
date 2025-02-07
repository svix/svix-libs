// this file is @generated
import { ApplicationOut, ApplicationOutUtil } from "../models/application_out";

export interface ListResponseApplicationOut {
  data: ApplicationOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseApplicationOutUtil {
  export function _fromJsonObject(object: any): ListResponseApplicationOut {
    return {
      data: object["data"].map((item) => ApplicationOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseApplicationOut): any {
    return {
      data: self.data.map((item) => ApplicationOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
