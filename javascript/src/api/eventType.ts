// this file is @generated
import {
  EventTypeImportOpenApiIn,
  EventTypeImportOpenApiInSerializer,
} from "../models/eventTypeImportOpenApiIn";
import {
  EventTypeImportOpenApiOut,
  EventTypeImportOpenApiOutSerializer,
} from "../models/eventTypeImportOpenApiOut";
import { EventTypeIn, EventTypeInSerializer } from "../models/eventTypeIn";
import { EventTypeOut, EventTypeOutSerializer } from "../models/eventTypeOut";
import { EventTypePatch, EventTypePatchSerializer } from "../models/eventTypePatch";
import { EventTypeUpdate, EventTypeUpdateSerializer } from "../models/eventTypeUpdate";
import {
  ListResponseEventTypeOut,
  ListResponseEventTypeOutSerializer,
} from "../models/listResponseEventTypeOut";
import { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface EventTypeListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
  /** When `true` archived (deleted but not expunged) items are included in the response. */
  includeArchived?: boolean;
  /** When `true` the full item (including the schema) is included in the response. */
  withContent?: boolean;
}

export interface EventTypeCreateOptions {
  idempotencyKey?: string;
}

export interface EventTypeImportOpenapiOptions {
  idempotencyKey?: string;
}

export interface EventTypeDeleteOptions {
  /** By default event types are archived when "deleted". Passing this to `true` deletes them entirely. */
  expunge?: boolean;
}

export class EventType {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Return the list of event types. */
  public list(options?: EventTypeListOptions): Promise<ListResponseEventTypeOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/event-type");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);
    request.setQueryParam("include_archived", options?.includeArchived);
    request.setQueryParam("with_content", options?.withContent);

    return request.send(
      this.requestCtx,
      ListResponseEventTypeOutSerializer._fromJsonObject
    );
  }

  /**
   * Create new or unarchive existing event type.
   *
   * Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
   * Endpoints filtering on the event type before archival will continue to filter on it.
   * This operation does not preserve the description and schemas.
   */
  public create(
    eventTypeIn: EventTypeIn,
    options?: EventTypeCreateOptions
  ): Promise<EventTypeOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/event-type");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(EventTypeInSerializer._toJsonObject(eventTypeIn));

    return request.send(this.requestCtx, EventTypeOutSerializer._fromJsonObject);
  }

  /**
   * Given an OpenAPI spec, create new or update existing event types.
   * If an existing `archived` event type is updated, it will be unarchived.
   *
   * The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
   * top-level.
   */
  public importOpenapi(
    eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
    options?: EventTypeImportOpenapiOptions
  ): Promise<EventTypeImportOpenApiOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/event-type/import/openapi");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      EventTypeImportOpenApiInSerializer._toJsonObject(eventTypeImportOpenApiIn)
    );

    return request.send(
      this.requestCtx,
      EventTypeImportOpenApiOutSerializer._fromJsonObject
    );
  }

  /** Get an event type. */
  public get(eventTypeName: string): Promise<EventTypeOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/event-type/{event_type_name}"
    );

    request.setPathParam("event_type_name", eventTypeName);

    return request.send(this.requestCtx, EventTypeOutSerializer._fromJsonObject);
  }

  /** Update an event type. */
  public update(
    eventTypeName: string,
    eventTypeUpdate: EventTypeUpdate
  ): Promise<EventTypeOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/event-type/{event_type_name}"
    );

    request.setPathParam("event_type_name", eventTypeName);
    request.setBody(EventTypeUpdateSerializer._toJsonObject(eventTypeUpdate));

    return request.send(this.requestCtx, EventTypeOutSerializer._fromJsonObject);
  }

  /**
   * Archive an event type.
   *
   * Endpoints already configured to filter on an event type will continue to do so after archival.
   * However, new messages can not be sent with it and endpoints can not filter on it.
   * An event type can be unarchived with the
   * [create operation](#operation/create_event_type_api_v1_event_type__post).
   */
  public delete(eventTypeName: string, options?: EventTypeDeleteOptions): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/event-type/{event_type_name}"
    );

    request.setPathParam("event_type_name", eventTypeName);
    request.setQueryParam("expunge", options?.expunge);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update an event type. */
  public patch(
    eventTypeName: string,
    eventTypePatch: EventTypePatch
  ): Promise<EventTypeOut> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/event-type/{event_type_name}"
    );

    request.setPathParam("event_type_name", eventTypeName);
    request.setBody(EventTypePatchSerializer._toJsonObject(eventTypePatch));

    return request.send(this.requestCtx, EventTypeOutSerializer._fromJsonObject);
  }
}
