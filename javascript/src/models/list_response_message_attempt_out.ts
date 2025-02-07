// this file is @generated
import { MessageAttemptOut, MessageAttemptOutUtil } from "../models/message_attempt_out";

export interface ListResponseMessageAttemptOut {
  data: MessageAttemptOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseMessageAttemptOutUtil {
  export function _fromJsonObject(object: any): ListResponseMessageAttemptOut {
    return {
      data: object["data"].map((item) => MessageAttemptOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseMessageAttemptOut): any {
    return {
      data: self.data.map((item) => MessageAttemptOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
