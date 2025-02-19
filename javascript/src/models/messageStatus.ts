// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
/**
 * The sending status of the message:
 * - Success = 0
 * - Pending = 1
 * - Fail = 2
 * - Sending = 3
 */
export enum MessageStatus {
  Success = 0,
  Pending = 1,
  Fail = 2,
  Sending = 3,
}

export const MessageStatusSerializer = {
  _fromJsonObject(object: any): MessageStatus {
    return object;
  },

  _toJsonObject(self: MessageStatus): any {
    return self;
  },
};
