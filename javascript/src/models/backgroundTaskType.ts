// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum BackgroundTaskType {
  EndpointReplay = "endpoint.replay",
  EndpointRecover = "endpoint.recover",
  ApplicationStats = "application.stats",
  MessageBroadcast = "message.broadcast",
  SdkGenerate = "sdk.generate",
  EventTypeAggregate = "event-type.aggregate",
  ApplicationPurgeContent = "application.purge_content",
}

export const BackgroundTaskTypeSerializer = {
  _fromJsonObject(object: any): BackgroundTaskType {
    return object;
  },

  _toJsonObject(self: BackgroundTaskType): any {
    return self;
  },
};
