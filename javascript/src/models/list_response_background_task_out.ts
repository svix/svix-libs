// this file is @generated
import { BackgroundTaskOut, BackgroundTaskOutUtil } from "../models/background_task_out";

export interface ListResponseBackgroundTaskOut {
  data: BackgroundTaskOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseBackgroundTaskOutUtil {
  export function _fromJsonObject(object: any): ListResponseBackgroundTaskOut {
    return {
      data: object["data"].map((item) => BackgroundTaskOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseBackgroundTaskOut): any {
    return {
      data: self.data.map((item) => BackgroundTaskOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
