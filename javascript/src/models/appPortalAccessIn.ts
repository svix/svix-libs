// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { ApplicationIn, ApplicationInSerializer } from "./applicationIn";

export interface AppPortalAccessIn {
  /**
   * Optionally creates a new application while generating the access link.
   *
   * If the application id or uid that is used in the path already exists, this argument is ignored.
   */
  application?: ApplicationIn | null;
  /**
   * How long the token will be valid for, in seconds.
   *
   * Valid values are between 1 hour and 7 days. The default is 7 days.
   */
  expiry?: number | null;
  /** The set of feature flags the created token will have access to. */
  featureFlags?: string[];
  /** Whether the app portal should be in read-only mode. */
  readOnly?: boolean | null;
}

export const AppPortalAccessInSerializer = {
  _fromJsonObject(object: any): AppPortalAccessIn {
    return {
      application: ApplicationInSerializer._fromJsonObject(object["application"]),
      expiry: object["expiry"],
      featureFlags: object["featureFlags"],
      readOnly: object["readOnly"],
    };
  },

  _toJsonObject(self: AppPortalAccessIn): any {
    return {
      application: self.application
        ? ApplicationInSerializer._toJsonObject(self.application)
        : undefined,
      expiry: self.expiry,
      featureFlags: self.featureFlags,
      readOnly: self.readOnly,
    };
  },
};
