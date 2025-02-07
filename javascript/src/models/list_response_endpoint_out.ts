// this file is @generated
import { EndpointOut, EndpointOutUtil } from "../models/endpoint_out";

export interface ListResponseEndpointOut {
  data: EndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseEndpointOutUtil {
  export function _fromJsonObject(object: any): ListResponseEndpointOut {
    return {
      data: object["data"].map((item) => EndpointOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseEndpointOut): any {
    return {
      data: self.data.map((item) => EndpointOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
