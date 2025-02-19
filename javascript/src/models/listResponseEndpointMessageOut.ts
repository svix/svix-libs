// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EndpointMessageOut, EndpointMessageOutSerializer } from "./endpointMessageOut";

export interface ListResponseEndpointMessageOut {
  data: EndpointMessageOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseEndpointMessageOutSerializer = {
  _fromJsonObject(object: any): ListResponseEndpointMessageOut {
    return {
      data: object["data"].map((item: EndpointMessageOut) =>
        EndpointMessageOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseEndpointMessageOut): any {
    return {
      data: self.data.map((item) => EndpointMessageOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
