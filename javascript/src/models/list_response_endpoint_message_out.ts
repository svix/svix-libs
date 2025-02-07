// this file is @generated
import {
  EndpointMessageOut,
  EndpointMessageOutUtil,
} from "../models/endpoint_message_out";

export interface ListResponseEndpointMessageOut {
  data: EndpointMessageOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseEndpointMessageOutUtil {
  export function _fromJsonObject(object: any): ListResponseEndpointMessageOut {
    return {
      data: object["data"].map((item) => EndpointMessageOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseEndpointMessageOut): any {
    return {
      data: self.data.map((item) => EndpointMessageOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
