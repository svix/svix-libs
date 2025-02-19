// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { BackgroundTaskOut, BackgroundTaskOutSerializer } from "./backgroundTaskOut";

export interface ListResponseBackgroundTaskOut {
  data: BackgroundTaskOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseBackgroundTaskOutSerializer = {
  _fromJsonObject(object: any): ListResponseBackgroundTaskOut {
    return {
      data: object["data"].map((item: BackgroundTaskOut) =>
        BackgroundTaskOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseBackgroundTaskOut): any {
    return {
      data: self.data.map((item) => BackgroundTaskOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
