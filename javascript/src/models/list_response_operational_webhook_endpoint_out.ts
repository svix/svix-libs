// this file is @generated
import {
  OperationalWebhookEndpointOut,
  OperationalWebhookEndpointOutUtil,
} from "../models/operational_webhook_endpoint_out";

export interface ListResponseOperationalWebhookEndpointOut {
  data: OperationalWebhookEndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseOperationalWebhookEndpointOutUtil {
  export function _fromJsonObject(
    object: any
  ): ListResponseOperationalWebhookEndpointOut {
    return {
      data: object["data"].map((item) =>
        OperationalWebhookEndpointOutUtil._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseOperationalWebhookEndpointOut): any {
    return {
      data: self.data.map((item) =>
        OperationalWebhookEndpointOutUtil._toJsonObject(item)
      ),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
