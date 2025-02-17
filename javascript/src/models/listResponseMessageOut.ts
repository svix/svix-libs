// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { MessageOut, MessageOutSerializer } from "./messageOut";

export interface ListResponseMessageOut {
  data: MessageOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseMessageOutSerializer = {
  _fromJsonObject(object: any): ListResponseMessageOut {
    return {
      data: object["data"].map((item: MessageOut) =>
        MessageOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseMessageOut): any {
    return {
      data: self.data.map((item) => MessageOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
