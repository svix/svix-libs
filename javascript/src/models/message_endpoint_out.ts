// this file is @generated
import { MessageStatus, MessageStatusUtil } from "../models/message_status";

export interface MessageEndpointOut {
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  createdAt: Date | null;
  /** An example endpoint name. */
  description: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  /** The ep's ID */
  id: string;
  nextAttempt?: Date | null | null;
  rateLimit?: number | null;
  status: MessageStatus;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  updatedAt: Date | null;
  url: string;
  version: number;
}

export namespace MessageEndpointOutUtil {
  export function _fromJsonObject(object: any): MessageEndpointOut {
    return {
      channels: object["channels"],
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      id: object["id"],
      nextAttempt: new Date(object["nextAttempt"]),
      rateLimit: object["rateLimit"],
      status: MessageStatusUtil._fromJsonObject(object["status"]),
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
      url: object["url"],
      version: object["version"],
    };
  }

  export function _toJsonObject(self: MessageEndpointOut): any {
    return {
      channels: self.channels,
      createdAt: self.createdAt,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      id: self.id,
      nextAttempt: self.nextAttempt,
      rateLimit: self.rateLimit,
      status: MessageStatusUtil._toJsonObject(self.status),
      uid: self.uid,
      updatedAt: self.updatedAt,
      url: self.url,
      version: self.version,
    };
  }
}
