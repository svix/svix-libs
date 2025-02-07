// this file is @generated
import { EventTypeOut, EventTypeOutUtil } from "../models/event_type_out";

export interface ListResponseEventTypeOut {
  data: EventTypeOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseEventTypeOutUtil {
  export function _fromJsonObject(object: any): ListResponseEventTypeOut {
    return {
      data: object["data"].map((item) => EventTypeOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseEventTypeOut): any {
    return {
      data: self.data.map((item) => EventTypeOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
