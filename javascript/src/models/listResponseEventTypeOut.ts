// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EventTypeOut, EventTypeOutSerializer } from "./eventTypeOut";

export interface ListResponseEventTypeOut {
  data: EventTypeOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseEventTypeOutSerializer = {
  _fromJsonObject(object: any): ListResponseEventTypeOut {
    return {
      data: object["data"].map((item: EventTypeOut) =>
        EventTypeOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseEventTypeOut): any {
    return {
      data: self.data.map((item) => EventTypeOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
