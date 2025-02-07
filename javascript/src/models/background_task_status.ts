// this file is @generated

export enum BackgroundTaskStatus {
  Running = "running",
  Finished = "finished",
  Failed = "failed",
}

export namespace BackgroundTaskStatusUtil {
  export function _fromJsonObject(object: any): BackgroundTaskStatus {
    return object;
  }

  export function _toJsonObject(self: BackgroundTaskStatus): any {
    return self;
  }
}
