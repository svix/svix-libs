// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface AppUsageStatsIn {
  /**
   * Specific app IDs or UIDs to aggregate stats for.
   *
   * Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
   */
  appIds?: string[] | null;
  since: Date | null;
  until: Date | null;
}

export const AppUsageStatsInSerializer = {
  _fromJsonObject(object: any): AppUsageStatsIn {
    return {
      appIds: object["appIds"],
      since: new Date(object["since"]),
      until: new Date(object["until"]),
    };
  },

  _toJsonObject(self: AppUsageStatsIn): any {
    return {
      appIds: self.appIds,
      since: self.since,
      until: self.until,
    };
  },
};
