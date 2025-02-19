// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { MessageStatus, MessageStatusSerializer } from "./messageStatus";
/** A model containing information on a given message plus additional fields on the last attempt for that message. */
export interface EndpointMessageOut {
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /** The Message's ID. */
  id: string;
  nextAttempt?: Date | null | null;
  payload: any;
  status: MessageStatus;
  tags?: string[] | null;
  timestamp: Date | null;
}

export const EndpointMessageOutSerializer = {
  _fromJsonObject(object: any): EndpointMessageOut {
    return {
      channels: object["channels"],
      eventId: object["eventId"],
      eventType: object["eventType"],
      id: object["id"],
      nextAttempt: new Date(object["nextAttempt"]),
      payload: object["payload"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      tags: object["tags"],
      timestamp: new Date(object["timestamp"]),
    };
  },

  _toJsonObject(self: EndpointMessageOut): any {
    return {
      channels: self.channels,
      eventId: self.eventId,
      eventType: self.eventType,
      id: self.id,
      nextAttempt: self.nextAttempt,
      payload: self.payload,
      status: MessageStatusSerializer._toJsonObject(self.status),
      tags: self.tags,
      timestamp: self.timestamp,
    };
  },
};
