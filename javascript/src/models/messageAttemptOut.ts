// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  MessageAttemptTriggerType,
  MessageAttemptTriggerTypeSerializer,
} from "./messageAttemptTriggerType";
import { MessageOut, MessageOutSerializer } from "./messageOut";
import { MessageStatus, MessageStatusSerializer } from "./messageStatus";

export interface MessageAttemptOut {
  /** The Endpoint's ID. */
  endpointId: string;
  /** The MessageAttempt's ID. */
  id: string;
  msg?: MessageOut | null;
  /** The Message's ID. */
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

export const MessageAttemptOutSerializer = {
  _fromJsonObject(object: any): MessageAttemptOut {
    return {
      endpointId: object["endpointId"],
      id: object["id"],
      msg: MessageOutSerializer._fromJsonObject(object["msg"]),
      msgId: object["msgId"],
      response: object["response"],
      responseDurationMs: object["responseDurationMs"],
      responseStatusCode: object["responseStatusCode"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      timestamp: new Date(object["timestamp"]),
      triggerType: MessageAttemptTriggerTypeSerializer._fromJsonObject(
        object["triggerType"]
      ),
      url: object["url"],
    };
  },

  _toJsonObject(self: MessageAttemptOut): any {
    return {
      endpointId: self.endpointId,
      id: self.id,
      msg: self.msg ? MessageOutSerializer._toJsonObject(self.msg) : undefined,
      msgId: self.msgId,
      response: self.response,
      responseDurationMs: self.responseDurationMs,
      responseStatusCode: self.responseStatusCode,
      status: MessageStatusSerializer._toJsonObject(self.status),
      timestamp: self.timestamp,
      triggerType: MessageAttemptTriggerTypeSerializer._toJsonObject(self.triggerType),
      url: self.url,
    };
  },
};
