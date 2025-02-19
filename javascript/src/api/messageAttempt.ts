// this file is @generated
import {
  ListResponseEndpointMessageOut,
  ListResponseEndpointMessageOutSerializer,
} from "../models/listResponseEndpointMessageOut";
import {
  ListResponseMessageAttemptOut,
  ListResponseMessageAttemptOutSerializer,
} from "../models/listResponseMessageAttemptOut";
import {
  ListResponseMessageEndpointOut,
  ListResponseMessageEndpointOutSerializer,
} from "../models/listResponseMessageEndpointOut";
import {
  MessageAttemptOut,
  MessageAttemptOutSerializer,
} from "../models/messageAttemptOut";
import { MessageStatus } from "../models/messageStatus";
import { StatusCodeClass } from "../models/statusCodeClass";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface MessageAttemptListByEndpointOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Filter response based on the HTTP status code */
  statusCodeClass?: StatusCodeClass;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the tag */
  tag?: string;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** When `true` attempt content is included in the response */
  withContent?: boolean;
  /** When `true`, the message information is included in the response */
  withMsg?: boolean;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export interface MessageAttemptListByMsgOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Filter response based on the HTTP status code */
  statusCodeClass?: StatusCodeClass;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the tag */
  tag?: string;
  /** Filter the attempts based on the attempted endpoint */
  endpointId?: string;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** When `true` attempt content is included in the response */
  withContent?: boolean;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export interface MessageAttemptListAttemptedMessagesOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the message tags */
  tag?: string;
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** When `true` message payloads are included in the response */
  withContent?: boolean;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export interface MessageAttemptListAttemptedDestinationsOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
}

export interface MessageAttemptResendOptions {
  idempotencyKey?: string;
}

export class MessageAttempt {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /**
   * List attempts by endpoint id
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public listByEndpoint(
    appId: string,
    endpointId: string,
    options?: MessageAttemptListByEndpointOptions
  ): Promise<ListResponseMessageAttemptOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("status", options?.status);
    request.setQueryParam("status_code_class", options?.statusCodeClass);
    request.setQueryParam("channel", options?.channel);
    request.setQueryParam("tag", options?.tag);
    request.setQueryParam("before", options?.before);
    request.setQueryParam("after", options?.after);
    request.setQueryParam("with_content", options?.withContent);
    request.setQueryParam("with_msg", options?.withMsg);
    request.setQueryParam("event_types", options?.eventTypes);

    return request.send(
      this.requestCtx,
      ListResponseMessageAttemptOutSerializer._fromJsonObject
    );
  }

  /**
   * List attempts by message ID.
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public listByMsg(
    appId: string,
    msgId: string,
    options?: MessageAttemptListByMsgOptions
  ): Promise<ListResponseMessageAttemptOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/attempt/msg/{msg_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("status", options?.status);
    request.setQueryParam("status_code_class", options?.statusCodeClass);
    request.setQueryParam("channel", options?.channel);
    request.setQueryParam("tag", options?.tag);
    request.setQueryParam("endpoint_id", options?.endpointId);
    request.setQueryParam("before", options?.before);
    request.setQueryParam("after", options?.after);
    request.setQueryParam("with_content", options?.withContent);
    request.setQueryParam("event_types", options?.eventTypes);

    return request.send(
      this.requestCtx,
      ListResponseMessageAttemptOutSerializer._fromJsonObject
    );
  }

  /**
   * List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
   *
   * The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public listAttemptedMessages(
    appId: string,
    endpointId: string,
    options?: MessageAttemptListAttemptedMessagesOptions
  ): Promise<ListResponseEndpointMessageOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("channel", options?.channel);
    request.setQueryParam("tag", options?.tag);
    request.setQueryParam("status", options?.status);
    request.setQueryParam("before", options?.before);
    request.setQueryParam("after", options?.after);
    request.setQueryParam("with_content", options?.withContent);
    request.setQueryParam("event_types", options?.eventTypes);

    return request.send(
      this.requestCtx,
      ListResponseEndpointMessageOutSerializer._fromJsonObject
    );
  }

  /** `msg_id`: Use a message id or a message `eventId` */
  public get(
    appId: string,
    msgId: string,
    attemptId: string
  ): Promise<MessageAttemptOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);
    request.setPathParam("attempt_id", attemptId);

    return request.send(this.requestCtx, MessageAttemptOutSerializer._fromJsonObject);
  }

  /**
   * Deletes the given attempt's response body.
   *
   * Useful when an endpoint accidentally returned sensitive content.
   * The message can't be replayed or resent once its payload has been deleted or expired.
   */
  public expungeContent(appId: string, msgId: string, attemptId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);
    request.setPathParam("attempt_id", attemptId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * List endpoints attempted by a given message.
   *
   * Additionally includes metadata about the latest message attempt.
   * By default, endpoints are listed in ascending order by ID.
   */
  public listAttemptedDestinations(
    appId: string,
    msgId: string,
    options?: MessageAttemptListAttemptedDestinationsOptions
  ): Promise<ListResponseMessageEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/msg/{msg_id}/endpoint"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);

    return request.send(
      this.requestCtx,
      ListResponseMessageEndpointOutSerializer._fromJsonObject
    );
  }

  /** Resend a message to the specified endpoint. */
  public resend(
    appId: string,
    msgId: string,
    endpointId: string,
    options?: MessageAttemptResendOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.sendNoResponseBody(this.requestCtx);
  }
}
