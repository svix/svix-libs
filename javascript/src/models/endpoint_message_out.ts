// this file is @generated
import { MessageStatus, MessageStatusUtil } from "../models/message_status";
/** A model containing information on a given message plus additional fields on the last attempt for that message. */
export interface EndpointMessageOut {
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /** The msg's ID */
  id: string;
  nextAttempt?: Date | null | null;
  payload: any;
  status: MessageStatus;
  tags?: string[] | null;
  timestamp: Date | null;
}

export namespace EndpointMessageOutUtil {
  export function _fromJsonObject(object: any): EndpointMessageOut {
    return {
      channels: object["channels"],
      eventId: object["eventId"],
      eventType: object["eventType"],
      id: object["id"],
      nextAttempt: new Date(object["nextAttempt"]),
      payload: object["payload"],
      status: MessageStatusUtil._fromJsonObject(object["status"]),
      tags: object["tags"],
      timestamp: new Date(object["timestamp"]),
    };
  }

  export function _toJsonObject(self: EndpointMessageOut): any {
    return {
      channels: self.channels,
      eventId: self.eventId,
      eventType: self.eventType,
      id: self.id,
      nextAttempt: self.nextAttempt,
      payload: self.payload,
      status: MessageStatusUtil._toJsonObject(self.status),
      tags: self.tags,
      timestamp: self.timestamp,
    };
  }
}
