// this file is @generated
import { ConnectorKind, ConnectorKindUtil } from "../models/connector_kind";

export interface TemplateOut {
  createdAt: Date | null;
  description: string;
  featureFlag?: string | null;
  filterTypes?: string[] | null;
  id: string;
  instructions: string;
  instructionsLink?: string | null;
  kind: ConnectorKind;
  logo: string;
  name: string;
  orgId: string;
  transformation: string;
  updatedAt: Date | null;
}

export namespace TemplateOutUtil {
  export function _fromJsonObject(object: any): TemplateOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      featureFlag: object["featureFlag"],
      filterTypes: object["filterTypes"],
      id: object["id"],
      instructions: object["instructions"],
      instructionsLink: object["instructionsLink"],
      kind: ConnectorKindUtil._fromJsonObject(object["kind"]),
      logo: object["logo"],
      name: object["name"],
      orgId: object["orgId"],
      transformation: object["transformation"],
      updatedAt: new Date(object["updatedAt"]),
    };
  }

  export function _toJsonObject(self: TemplateOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      featureFlag: self.featureFlag,
      filterTypes: self.filterTypes,
      id: self.id,
      instructions: self.instructions,
      instructionsLink: self.instructionsLink,
      kind: ConnectorKindUtil._toJsonObject(self.kind),
      logo: self.logo,
      name: self.name,
      orgId: self.orgId,
      transformation: self.transformation,
      updatedAt: self.updatedAt,
    };
  }
}
