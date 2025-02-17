// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  EventTypeImportOpenApiOutData,
  EventTypeImportOpenApiOutDataSerializer,
} from "./eventTypeImportOpenApiOutData";

export interface EventTypeImportOpenApiOut {
  data: EventTypeImportOpenApiOutData;
}

export const EventTypeImportOpenApiOutSerializer = {
  _fromJsonObject(object: any): EventTypeImportOpenApiOut {
    return {
      data: EventTypeImportOpenApiOutDataSerializer._fromJsonObject(object["data"]),
    };
  },

  _toJsonObject(self: EventTypeImportOpenApiOut): any {
    return {
      data: EventTypeImportOpenApiOutDataSerializer._toJsonObject(self.data),
    };
  },
};
