// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
/**
 * The reason an attempt was made:
 * - Scheduled = 0
 * - Manual = 1
 */
export enum MessageAttemptTriggerType {
  Scheduled = 0,
  Manual = 1,
}

export const MessageAttemptTriggerTypeSerializer = {
  _fromJsonObject(object: any): MessageAttemptTriggerType {
    return object;
  },

  _toJsonObject(self: MessageAttemptTriggerType): any {
    return self;
  },
};
