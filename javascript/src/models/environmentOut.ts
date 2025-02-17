// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EventTypeOut, EventTypeOutSerializer } from "./eventTypeOut";
import { TemplateOut, TemplateOutSerializer } from "./templateOut";

export interface EnvironmentOut {
  createdAt: Date | null;
  eventTypes: EventTypeOut[];
  settings: any | null;
  transformationTemplates: TemplateOut[];
  version?: number;
}

export const EnvironmentOutSerializer = {
  _fromJsonObject(object: any): EnvironmentOut {
    return {
      createdAt: new Date(object["createdAt"]),
      eventTypes: object["eventTypes"].map((item: EventTypeOut) =>
        EventTypeOutSerializer._fromJsonObject(item)
      ),
      settings: object["settings"],
      transformationTemplates: object["transformationTemplates"].map(
        (item: TemplateOut) => TemplateOutSerializer._fromJsonObject(item)
      ),
      version: object["version"],
    };
  },

  _toJsonObject(self: EnvironmentOut): any {
    return {
      createdAt: self.createdAt,
      eventTypes: self.eventTypes.map((item) =>
        EventTypeOutSerializer._toJsonObject(item)
      ),
      settings: self.settings,
      transformationTemplates: self.transformationTemplates.map((item) =>
        TemplateOutSerializer._toJsonObject(item)
      ),
      version: self.version,
    };
  },
};
