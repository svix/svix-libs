// this file is @generated

export enum BackgroundTaskType {
  EndpointReplay = "endpoint.replay",
  EndpointRecover = "endpoint.recover",
  ApplicationStats = "application.stats",
  MessageBroadcast = "message.broadcast",
  SdkGenerate = "sdk.generate",
  EventTypeAggregate = "event-type.aggregate",
}

export namespace BackgroundTaskTypeUtil {
  export function _fromJsonObject(object: any): BackgroundTaskType {
    return object;
  }

  export function _toJsonObject(self: BackgroundTaskType): any {
    return self;
  }
}
