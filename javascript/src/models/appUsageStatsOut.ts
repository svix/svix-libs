// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import { BackgroundTaskType, BackgroundTaskTypeSerializer } from "./backgroundTaskType";

export interface AppUsageStatsOut {
  /** The QueueBackgroundTask's ID. */
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
  /**
   * Any app IDs or UIDs received in the request that weren't found.
   *
   * Stats will be produced for all the others.
   */
  unresolvedAppIds: string[];
}

export const AppUsageStatsOutSerializer = {
  _fromJsonObject(object: any): AppUsageStatsOut {
    return {
      id: object["id"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
      unresolvedAppIds: object["unresolvedAppIds"],
    };
  },

  _toJsonObject(self: AppUsageStatsOut): any {
    return {
      id: self.id,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
      unresolvedAppIds: self.unresolvedAppIds,
    };
  },
};
