// this file is @generated
import {
  EventTypeFromOpenApi,
  EventTypeFromOpenApiUtil,
} from "../models/event_type_from_open_api";

export interface EventTypeImportOpenApiOutData {
  modified: string[];
  toModify?: EventTypeFromOpenApi[] | null;
}

export namespace EventTypeImportOpenApiOutDataUtil {
  export function _fromJsonObject(object: any): EventTypeImportOpenApiOutData {
    return {
      modified: object["modified"],
      toModify: object["to_modify"].map((item) =>
        EventTypeFromOpenApiUtil._fromJsonObject(item)
      ),
    };
  }

  export function _toJsonObject(self: EventTypeImportOpenApiOutData): any {
    return {
      modified: self.modified,
      to_modify: self.toModify.map((item) =>
        EventTypeFromOpenApiUtil._toJsonObject(item)
      ),
    };
  }
}
