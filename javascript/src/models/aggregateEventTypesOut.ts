// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import { BackgroundTaskType, BackgroundTaskTypeSerializer } from "./backgroundTaskType";

export interface AggregateEventTypesOut {
  /** The QueueBackgroundTask's ID. */
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
}

export const AggregateEventTypesOutSerializer = {
  _fromJsonObject(object: any): AggregateEventTypesOut {
    return {
      id: object["id"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
    };
  },

  _toJsonObject(self: AggregateEventTypesOut): any {
    return {
      id: self.id,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
    };
  },
};
