// this file is @generated
import { ConnectorIn, ConnectorInUtil } from "../models/connector_in";
import { EventTypeIn, EventTypeInUtil } from "../models/event_type_in";

export interface EnvironmentIn {
  connectors?: ConnectorIn[] | null;
  eventTypes?: EventTypeIn[] | null;
  settings?: any | null;
}

export namespace EnvironmentInUtil {
  export function _fromJsonObject(object: any): EnvironmentIn {
    return {
      connectors: object["connectors"].map((item) =>
        ConnectorInUtil._fromJsonObject(item)
      ),
      eventTypes: object["eventTypes"].map((item) =>
        EventTypeInUtil._fromJsonObject(item)
      ),
      settings: object["settings"],
    };
  }

  export function _toJsonObject(self: EnvironmentIn): any {
    return {
      connectors: self.connectors.map((item) => ConnectorInUtil._toJsonObject(item)),
      eventTypes: self.eventTypes.map((item) => EventTypeInUtil._toJsonObject(item)),
      settings: self.settings,
    };
  }
}
