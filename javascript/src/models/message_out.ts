// this file is @generated

export interface MessageOut {
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /** The msg's ID */
  id: string;
  payload: any;
  tags?: string[] | null;
  timestamp: Date | null;
}

export namespace MessageOutUtil {
  export function _fromJsonObject(object: any): MessageOut {
    return {
      channels: object["channels"],
      eventId: object["eventId"],
      eventType: object["eventType"],
      id: object["id"],
      payload: object["payload"],
      tags: object["tags"],
      timestamp: new Date(object["timestamp"]),
    };
  }

  export function _toJsonObject(self: MessageOut): any {
    return {
      channels: self.channels,
      eventId: self.eventId,
      eventType: self.eventType,
      id: self.id,
      payload: self.payload,
      tags: self.tags,
      timestamp: self.timestamp,
    };
  }
}
