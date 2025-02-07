// this file is @generated
import {
  MessageEndpointOut,
  MessageEndpointOutUtil,
} from "../models/message_endpoint_out";

export interface ListResponseMessageEndpointOut {
  data: MessageEndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseMessageEndpointOutUtil {
  export function _fromJsonObject(object: any): ListResponseMessageEndpointOut {
    return {
      data: object["data"].map((item) => MessageEndpointOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseMessageEndpointOut): any {
    return {
      data: self.data.map((item) => MessageEndpointOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
