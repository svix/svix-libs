// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  OperationalWebhookEndpointOut,
  OperationalWebhookEndpointOutSerializer,
} from "./operationalWebhookEndpointOut";

export interface ListResponseOperationalWebhookEndpointOut {
  data: OperationalWebhookEndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseOperationalWebhookEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseOperationalWebhookEndpointOut {
    return {
      data: object["data"].map((item: OperationalWebhookEndpointOut) =>
        OperationalWebhookEndpointOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseOperationalWebhookEndpointOut): any {
    return {
      data: self.data.map((item) =>
        OperationalWebhookEndpointOutSerializer._toJsonObject(item)
      ),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
