// this file is @generated
import { ConnectorKind, ConnectorKindUtil } from "../models/connector_kind";

export interface ConnectorIn {
  description?: string;
  featureFlag?: string | null;
  filterTypes?: string[] | null;
  instructions?: string;
  instructionsLink?: string | null;
  kind?: ConnectorKind;
  logo: string;
  name: string;
  transformation: string;
}

export namespace ConnectorInUtil {
  export function _fromJsonObject(object: any): ConnectorIn {
    return {
      description: object["description"],
      featureFlag: object["featureFlag"],
      filterTypes: object["filterTypes"],
      instructions: object["instructions"],
      instructionsLink: object["instructionsLink"],
      kind: ConnectorKindUtil._fromJsonObject(object["kind"]),
      logo: object["logo"],
      name: object["name"],
      transformation: object["transformation"],
    };
  }

  export function _toJsonObject(self: ConnectorIn): any {
    return {
      description: self.description,
      featureFlag: self.featureFlag,
      filterTypes: self.filterTypes,
      instructions: self.instructions,
      instructionsLink: self.instructionsLink,
      kind: ConnectorKindUtil._toJsonObject(self.kind),
      logo: self.logo,
      name: self.name,
      transformation: self.transformation,
    };
  }
}
