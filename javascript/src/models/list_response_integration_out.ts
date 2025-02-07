// this file is @generated
import { IntegrationOut, IntegrationOutUtil } from "../models/integration_out";

export interface ListResponseIntegrationOut {
  data: IntegrationOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export namespace ListResponseIntegrationOutUtil {
  export function _fromJsonObject(object: any): ListResponseIntegrationOut {
    return {
      data: object["data"].map((item) => IntegrationOutUtil._fromJsonObject(item)),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  }

  export function _toJsonObject(self: ListResponseIntegrationOut): any {
    return {
      data: self.data.map((item) => IntegrationOutUtil._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  }
}
