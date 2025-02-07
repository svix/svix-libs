// this file is @generated
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

export namespace MessageStatusUtil {
  export function _fromJsonObject(object: any): MessageStatus {
    return object;
  }

  export function _toJsonObject(self: MessageStatus): any {
    return self;
  }
}
