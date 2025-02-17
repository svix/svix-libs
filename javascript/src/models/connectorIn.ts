// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

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

export const ConnectorInSerializer = {
  _fromJsonObject(object: any): ConnectorIn {
    return {
      description: object["description"],
      featureFlag: object["featureFlag"],
      filterTypes: object["filterTypes"],
      instructions: object["instructions"],
      instructionsLink: object["instructionsLink"],
      kind: ConnectorKindSerializer._fromJsonObject(object["kind"]),
      logo: object["logo"],
      name: object["name"],
      transformation: object["transformation"],
    };
  },

  _toJsonObject(self: ConnectorIn): any {
    return {
      description: self.description,
      featureFlag: self.featureFlag,
      filterTypes: self.filterTypes,
      instructions: self.instructions,
      instructionsLink: self.instructionsLink,
      kind: self.kind ? ConnectorKindSerializer._toJsonObject(self.kind) : undefined,
      logo: self.logo,
      name: self.name,
      transformation: self.transformation,
    };
  },
};
