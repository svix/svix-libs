// this file is @generated
import {
  MessageAttemptTriggerType,
  MessageAttemptTriggerTypeUtil,
} from "../models/message_attempt_trigger_type";
import { MessageOut, MessageOutUtil } from "../models/message_out";
import { MessageStatus, MessageStatusUtil } from "../models/message_status";

export interface MessageAttemptOut {
  /** The ep's ID */
  endpointId: string;
  /** The attempt's ID */
  id: string;
  msg?: MessageOut | null;
  /** The msg's ID */
  msgId: string;
  response: string;
  /** Response duration in milliseconds. */
  responseDurationMs: number;
  responseStatusCode: number;
  status: MessageStatus;
  timestamp: Date | null;
  triggerType: MessageAttemptTriggerType;
  url: string;
}

export namespace MessageAttemptOutUtil {
  export function _fromJsonObject(object: any): MessageAttemptOut {
    return {
      endpointId: object["endpointId"],
      id: object["id"],
      msg: MessageOutUtil._fromJsonObject(object["msg"]),
      msgId: object["msgId"],
      response: object["response"],
      responseDurationMs: object["responseDurationMs"],
      responseStatusCode: object["responseStatusCode"],
      status: MessageStatusUtil._fromJsonObject(object["status"]),
      timestamp: new Date(object["timestamp"]),
      triggerType: MessageAttemptTriggerTypeUtil._fromJsonObject(object["triggerType"]),
      url: object["url"],
    };
  }

  export function _toJsonObject(self: MessageAttemptOut): any {
    return {
      endpointId: self.endpointId,
      id: self.id,
      msg: MessageOutUtil._toJsonObject(self.msg),
      msgId: self.msgId,
      response: self.response,
      responseDurationMs: self.responseDurationMs,
      responseStatusCode: self.responseStatusCode,
      status: MessageStatusUtil._toJsonObject(self.status),
      timestamp: self.timestamp,
      triggerType: MessageAttemptTriggerTypeUtil._toJsonObject(self.triggerType),
      url: self.url,
    };
  }
}
