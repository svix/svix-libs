// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { MessageEndpointOut, MessageEndpointOutSerializer } from "./messageEndpointOut";

export interface ListResponseMessageEndpointOut {
  data: MessageEndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseMessageEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseMessageEndpointOut {
    return {
      data: object["data"].map((item: MessageEndpointOut) =>
        MessageEndpointOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseMessageEndpointOut): any {
    return {
      data: self.data.map((item) => MessageEndpointOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
