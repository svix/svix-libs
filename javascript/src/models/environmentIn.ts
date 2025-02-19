// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { ConnectorIn, ConnectorInSerializer } from "./connectorIn";
import { EventTypeIn, EventTypeInSerializer } from "./eventTypeIn";

export interface EnvironmentIn {
  connectors?: ConnectorIn[] | null;
  eventTypes?: EventTypeIn[] | null;
  settings?: any | null;
}

export const EnvironmentInSerializer = {
  _fromJsonObject(object: any): EnvironmentIn {
    return {
      connectors: object["connectors"].map((item: ConnectorIn) =>
        ConnectorInSerializer._fromJsonObject(item)
      ),
      eventTypes: object["eventTypes"].map((item: EventTypeIn) =>
        EventTypeInSerializer._fromJsonObject(item)
      ),
      settings: object["settings"],
    };
  },

  _toJsonObject(self: EnvironmentIn): any {
    return {
      connectors: self.connectors?.map((item) =>
        ConnectorInSerializer._toJsonObject(item)
      ),
      eventTypes: self.eventTypes?.map((item) =>
        EventTypeInSerializer._toJsonObject(item)
      ),
      settings: self.settings,
    };
  },
};
