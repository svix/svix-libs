// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  EventTypeFromOpenApi,
  EventTypeFromOpenApiSerializer,
} from "./eventTypeFromOpenApi";

export interface EventTypeImportOpenApiOutData {
  modified: string[];
  toModify?: EventTypeFromOpenApi[] | null;
}

export const EventTypeImportOpenApiOutDataSerializer = {
  _fromJsonObject(object: any): EventTypeImportOpenApiOutData {
    return {
      modified: object["modified"],
      toModify: object["to_modify"].map((item: EventTypeFromOpenApi) =>
        EventTypeFromOpenApiSerializer._fromJsonObject(item)
      ),
    };
  },

  _toJsonObject(self: EventTypeImportOpenApiOutData): any {
    return {
      modified: self.modified,
      to_modify: self.toModify?.map((item) =>
        EventTypeFromOpenApiSerializer._toJsonObject(item)
      ),
    };
  },
};
