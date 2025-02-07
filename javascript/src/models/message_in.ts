// this file is @generated
import { ApplicationIn, ApplicationInUtil } from "../models/application_in";

export interface MessageIn {
  /**
   * Optionally creates a new application alongside the message.
   *
   * If the application id or uid that is used in the path already exists, this argument is ignored.
   */
  application?: ApplicationIn | null;
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /**
   * JSON payload to send as the request body of the webhook.
   *
   * We also support sending non-JSON payloads. Please contact us for more information.
   */
  payload: any;
  /** Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`. */
  payloadRetentionHours?: number | null;
  /** Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`. */
  payloadRetentionPeriod?: number | null;
  /** List of free-form tags that can be filtered by when listing messages */
  tags?: string[] | null;
  /** Extra parameters to pass to Transformations (for future use) */
  transformationsParams?: any | null;
}

export namespace MessageInUtil {
  export function _fromJsonObject(object: any): MessageIn {
    return {
      application: ApplicationInUtil._fromJsonObject(object["application"]),
      channels: object["channels"],
      eventId: object["eventId"],
      eventType: object["eventType"],
      payload: object["payload"],
      payloadRetentionHours: object["payloadRetentionHours"],
      payloadRetentionPeriod: object["payloadRetentionPeriod"],
      tags: object["tags"],
      transformationsParams: object["transformationsParams"],
    };
  }

  export function _toJsonObject(self: MessageIn): any {
    return {
      application: ApplicationInUtil._toJsonObject(self.application),
      channels: self.channels,
      eventId: self.eventId,
      eventType: self.eventType,
      payload: self.payload,
      payloadRetentionHours: self.payloadRetentionHours,
      payloadRetentionPeriod: self.payloadRetentionPeriod,
      tags: self.tags,
      transformationsParams: self.transformationsParams,
    };
  }
}
