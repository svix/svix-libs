// this file is @generated
import {
  EventTypeImportOpenApiOutData,
  EventTypeImportOpenApiOutDataUtil,
} from "../models/event_type_import_open_api_out_data";

export interface EventTypeImportOpenApiOut {
  data: EventTypeImportOpenApiOutData;
}

export namespace EventTypeImportOpenApiOutUtil {
  export function _fromJsonObject(object: any): EventTypeImportOpenApiOut {
    return {
      data: EventTypeImportOpenApiOutDataUtil._fromJsonObject(object["data"]),
    };
  }

  export function _toJsonObject(self: EventTypeImportOpenApiOut): any {
    return {
      data: EventTypeImportOpenApiOutDataUtil._toJsonObject(self.data),
    };
  }
}
