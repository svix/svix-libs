// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

export interface TemplateOut {
  createdAt: Date | null;
  description: string;
  featureFlag?: string | null;
  filterTypes?: string[] | null;
  /** The TransformationTemplate's ID. */
  id: string;
  instructions: string;
  instructionsLink?: string | null;
  kind: ConnectorKind;
  logo: string;
  name: string;
  /** The Organization's ID. */
  orgId: string;
  transformation: string;
  updatedAt: Date | null;
}

export const TemplateOutSerializer = {
  _fromJsonObject(object: any): TemplateOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      featureFlag: object["featureFlag"],
      filterTypes: object["filterTypes"],
      id: object["id"],
      instructions: object["instructions"],
      instructionsLink: object["instructionsLink"],
      kind: ConnectorKindSerializer._fromJsonObject(object["kind"]),
      logo: object["logo"],
      name: object["name"],
      orgId: object["orgId"],
      transformation: object["transformation"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: TemplateOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      featureFlag: self.featureFlag,
      filterTypes: self.filterTypes,
      id: self.id,
      instructions: self.instructions,
      instructionsLink: self.instructionsLink,
      kind: ConnectorKindSerializer._toJsonObject(self.kind),
      logo: self.logo,
      name: self.name,
      orgId: self.orgId,
      transformation: self.transformation,
      updatedAt: self.updatedAt,
    };
  },
};
