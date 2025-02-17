// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { ApplicationOut, ApplicationOutSerializer } from "./applicationOut";

export interface ListResponseApplicationOut {
  data: ApplicationOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseApplicationOutSerializer = {
  _fromJsonObject(object: any): ListResponseApplicationOut {
    return {
      data: object["data"].map((item: ApplicationOut) =>
        ApplicationOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseApplicationOut): any {
    return {
      data: self.data.map((item) => ApplicationOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
