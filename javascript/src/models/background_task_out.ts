// this file is @generated
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusUtil,
} from "../models/background_task_status";
import {
  BackgroundTaskType,
  BackgroundTaskTypeUtil,
} from "../models/background_task_type";

export interface BackgroundTaskOut {
  data: any;
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
}

export namespace BackgroundTaskOutUtil {
  export function _fromJsonObject(object: any): BackgroundTaskOut {
    return {
      data: object["data"],
      id: object["id"],
      status: BackgroundTaskStatusUtil._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeUtil._fromJsonObject(object["task"]),
    };
  }

  export function _toJsonObject(self: BackgroundTaskOut): any {
    return {
      data: self.data,
      id: self.id,
      status: BackgroundTaskStatusUtil._toJsonObject(self.status),
      task: BackgroundTaskTypeUtil._toJsonObject(self.task),
    };
  }
}
