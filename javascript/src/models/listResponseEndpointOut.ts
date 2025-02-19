// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EndpointOut, EndpointOutSerializer } from "./endpointOut";

export interface ListResponseEndpointOut {
  data: EndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseEndpointOut {
    return {
      data: object["data"].map((item: EndpointOut) =>
        EndpointOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseEndpointOut): any {
    return {
      data: self.data.map((item) => EndpointOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
