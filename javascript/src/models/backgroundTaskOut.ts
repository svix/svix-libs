// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import { BackgroundTaskType, BackgroundTaskTypeSerializer } from "./backgroundTaskType";

export interface BackgroundTaskOut {
  data: any;
  /** The QueueBackgroundTask's ID. */
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
}

export const BackgroundTaskOutSerializer = {
  _fromJsonObject(object: any): BackgroundTaskOut {
    return {
      data: object["data"],
      id: object["id"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
    };
  },

  _toJsonObject(self: BackgroundTaskOut): any {
    return {
      data: self.data,
      id: self.id,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
    };
  },
};
