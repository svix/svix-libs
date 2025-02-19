// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { IntegrationOut, IntegrationOutSerializer } from "./integrationOut";

export interface ListResponseIntegrationOut {
  data: IntegrationOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseIntegrationOutSerializer = {
  _fromJsonObject(object: any): ListResponseIntegrationOut {
    return {
      data: object["data"].map((item: IntegrationOut) =>
        IntegrationOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseIntegrationOut): any {
    return {
      data: self.data.map((item) => IntegrationOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
