// this file is @generated
/**
 * The reason an attempt was made:
 * - Scheduled = 0
 * - Manual = 1
 */
export enum MessageAttemptTriggerType {
  Scheduled = 0,
  Manual = 1,
}

export namespace MessageAttemptTriggerTypeUtil {
  export function _fromJsonObject(object: any): MessageAttemptTriggerType {
    return object;
  }

  export function _toJsonObject(self: MessageAttemptTriggerType): any {
    return self;
  }
}
