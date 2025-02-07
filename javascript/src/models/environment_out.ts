// this file is @generated
import { EventTypeOut, EventTypeOutUtil } from "../models/event_type_out";
import { TemplateOut, TemplateOutUtil } from "../models/template_out";

export interface EnvironmentOut {
  createdAt: Date | null;
  eventTypes: EventTypeOut[];
  settings: any | null;
  transformationTemplates: TemplateOut[];
  version?: number;
}

export namespace EnvironmentOutUtil {
  export function _fromJsonObject(object: any): EnvironmentOut {
    return {
      createdAt: new Date(object["createdAt"]),
      eventTypes: object["eventTypes"].map((item) =>
        EventTypeOutUtil._fromJsonObject(item)
      ),
      settings: object["settings"],
      transformationTemplates: object["transformationTemplates"].map((item) =>
        TemplateOutUtil._fromJsonObject(item)
      ),
      version: object["version"],
    };
  }

  export function _toJsonObject(self: EnvironmentOut): any {
    return {
      createdAt: self.createdAt,
      eventTypes: self.eventTypes.map((item) => EventTypeOutUtil._toJsonObject(item)),
      settings: self.settings,
      transformationTemplates: self.transformationTemplates.map((item) =>
        TemplateOutUtil._toJsonObject(item)
      ),
      version: self.version,
    };
  }
}
