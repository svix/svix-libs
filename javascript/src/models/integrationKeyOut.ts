// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IntegrationKeyOut {
  key: string;
}

export const IntegrationKeyOutSerializer = {
  _fromJsonObject(object: any): IntegrationKeyOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: IntegrationKeyOut): any {
    return {
      key: self.key,
    };
  },
};
