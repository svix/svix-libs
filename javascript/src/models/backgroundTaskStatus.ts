// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum BackgroundTaskStatus {
  Running = "running",
  Finished = "finished",
  Failed = "failed",
}

export const BackgroundTaskStatusSerializer = {
  _fromJsonObject(object: any): BackgroundTaskStatus {
    return object;
  },

  _toJsonObject(self: BackgroundTaskStatus): any {
    return self;
  },
};
