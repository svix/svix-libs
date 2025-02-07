// this file is @generated
import { MessageOut, MessageOutUtil } from "../models/message_out";

export interface ListResponseMessageOut {
  data: MessageOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseMessageOutUtil {
  export function _fromJsonObject(object: any): ListResponseMessageOut {
    return {
      data: object["data"].map((item) => MessageOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseMessageOut): any {
    return {
      data: self.data.map((item) => MessageOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
