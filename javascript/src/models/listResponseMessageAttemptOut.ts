// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { MessageAttemptOut, MessageAttemptOutSerializer } from "./messageAttemptOut";

export interface ListResponseMessageAttemptOut {
  data: MessageAttemptOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseMessageAttemptOutSerializer = {
  _fromJsonObject(object: any): ListResponseMessageAttemptOut {
    return {
      data: object["data"].map((item: MessageAttemptOut) =>
        MessageAttemptOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseMessageAttemptOut): any {
    return {
      data: self.data.map((item) => MessageAttemptOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
