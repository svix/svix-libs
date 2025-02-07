// this file is @generated
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusUtil,
} from "../models/background_task_status";
import {
  BackgroundTaskType,
  BackgroundTaskTypeUtil,
} from "../models/background_task_type";

export interface AppUsageStatsOut {
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

export namespace AppUsageStatsOutUtil {
  export function _fromJsonObject(object: any): AppUsageStatsOut {
    return {
      id: object["id"],
      status: BackgroundTaskStatusUtil._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeUtil._fromJsonObject(object["task"]),
      unresolvedAppIds: object["unresolvedAppIds"],
    };
  }

  export function _toJsonObject(self: AppUsageStatsOut): any {
    return {
      id: self.id,
      status: BackgroundTaskStatusUtil._toJsonObject(self.status),
      task: BackgroundTaskTypeUtil._toJsonObject(self.task),
      unresolvedAppIds: self.unresolvedAppIds,
    };
  }
}
