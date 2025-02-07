// this file is @generated
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusUtil,
} from "../models/background_task_status";
import {
  BackgroundTaskType,
  BackgroundTaskTypeUtil,
} from "../models/background_task_type";

export interface RecoverOut {
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
}

export namespace RecoverOutUtil {
  export function _fromJsonObject(object: any): RecoverOut {
    return {
      id: object["id"],
      status: BackgroundTaskStatusUtil._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeUtil._fromJsonObject(object["task"]),
    };
  }

  export function _toJsonObject(self: RecoverOut): any {
    return {
      id: self.id,
      status: BackgroundTaskStatusUtil._toJsonObject(self.status),
      task: BackgroundTaskTypeUtil._toJsonObject(self.task),
    };
  }
}
