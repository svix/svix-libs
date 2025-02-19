// this file is @generated
import {
  EndpointHeadersIn,
  EndpointHeadersInSerializer,
} from "../models/endpointHeadersIn";
import {
  EndpointHeadersOut,
  EndpointHeadersOutSerializer,
} from "../models/endpointHeadersOut";
import {
  EndpointHeadersPatchIn,
  EndpointHeadersPatchInSerializer,
} from "../models/endpointHeadersPatchIn";
import { EndpointIn, EndpointInSerializer } from "../models/endpointIn";
import { EndpointOut, EndpointOutSerializer } from "../models/endpointOut";
import { EndpointPatch, EndpointPatchSerializer } from "../models/endpointPatch";
import {
  EndpointSecretOut,
  EndpointSecretOutSerializer,
} from "../models/endpointSecretOut";
import {
  EndpointSecretRotateIn,
  EndpointSecretRotateInSerializer,
} from "../models/endpointSecretRotateIn";
import { EndpointStats, EndpointStatsSerializer } from "../models/endpointStats";
import {
  EndpointTransformationIn,
  EndpointTransformationInSerializer,
} from "../models/endpointTransformationIn";
import {
  EndpointTransformationOut,
  EndpointTransformationOutSerializer,
} from "../models/endpointTransformationOut";
import { EndpointUpdate, EndpointUpdateSerializer } from "../models/endpointUpdate";
import { EventExampleIn, EventExampleInSerializer } from "../models/eventExampleIn";
import {
  ListResponseEndpointOut,
  ListResponseEndpointOutSerializer,
} from "../models/listResponseEndpointOut";
import { MessageOut, MessageOutSerializer } from "../models/messageOut";
import { Ordering } from "../models/ordering";
import { RecoverIn, RecoverInSerializer } from "../models/recoverIn";
import { RecoverOut, RecoverOutSerializer } from "../models/recoverOut";
import { ReplayIn, ReplayInSerializer } from "../models/replayIn";
import { ReplayOut, ReplayOutSerializer } from "../models/replayOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface EndpointListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface EndpointCreateOptions {
  idempotencyKey?: string;
}

export interface EndpointRecoverOptions {
  idempotencyKey?: string;
}

export interface EndpointReplayMissingOptions {
  idempotencyKey?: string;
}

export interface EndpointRotateSecretOptions {
  idempotencyKey?: string;
}

export interface EndpointSendExampleOptions {
  idempotencyKey?: string;
}

export interface EndpointGetStatsOptions {
  /** Filter the range to data starting from this date. */
  since?: Date | null;
  /** Filter the range to data ending by this date. */
  until?: Date | null;
}

export class Endpoint {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List the application's endpoints. */
  public list(
    appId: string,
    options?: EndpointListOptions
  ): Promise<ListResponseEndpointOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}/endpoint");

    request.setPathParam("app_id", appId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseEndpointOutSerializer._fromJsonObject
    );
  }

  /**
   * Create a new endpoint for the application.
   *
   * When `secret` is `null` the secret is automatically generated (recommended).
   */
  public create(
    appId: string,
    endpointIn: EndpointIn,
    options?: EndpointCreateOptions
  ): Promise<EndpointOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app/{app_id}/endpoint");

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(EndpointInSerializer._toJsonObject(endpointIn));

    return request.send(this.requestCtx, EndpointOutSerializer._fromJsonObject);
  }

  /** Get an endpoint. */
  public get(appId: string, endpointId: string): Promise<EndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, EndpointOutSerializer._fromJsonObject);
  }

  /** Update an endpoint. */
  public update(
    appId: string,
    endpointId: string,
    endpointUpdate: EndpointUpdate
  ): Promise<EndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(EndpointUpdateSerializer._toJsonObject(endpointUpdate));

    return request.send(this.requestCtx, EndpointOutSerializer._fromJsonObject);
  }

  /** Delete an endpoint. */
  public delete(appId: string, endpointId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update an endpoint. */
  public patch(
    appId: string,
    endpointId: string,
    endpointPatch: EndpointPatch
  ): Promise<EndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(EndpointPatchSerializer._toJsonObject(endpointPatch));

    return request.send(this.requestCtx, EndpointOutSerializer._fromJsonObject);
  }

  /** Get the additional headers to be sent with the webhook. */
  public getHeaders(appId: string, endpointId: string): Promise<EndpointHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, EndpointHeadersOutSerializer._fromJsonObject);
  }

  /** Set the additional headers to be sent with the webhook. */
  public updateHeaders(
    appId: string,
    endpointId: string,
    endpointHeadersIn: EndpointHeadersIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(EndpointHeadersInSerializer._toJsonObject(endpointHeadersIn));

    return request.sendNoResponseBody(this.requestCtx);
  }

  public headersUpdate(
    appId: string,
    endpointId: string,
    endpointHeadersIn: EndpointHeadersIn
  ): Promise<void> {
    return this.updateHeaders(appId, endpointId, endpointHeadersIn);
  }

  /** Partially set the additional headers to be sent with the webhook. */
  public patchHeaders(
    appId: string,
    endpointId: string,
    endpointHeadersPatchIn: EndpointHeadersPatchIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      EndpointHeadersPatchInSerializer._toJsonObject(endpointHeadersPatchIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }

  public headersPatch(
    appId: string,
    endpointId: string,
    endpointHeadersPatchIn: EndpointHeadersPatchIn
  ): Promise<void> {
    return this.patchHeaders(appId, endpointId, endpointHeadersPatchIn);
  }

  /**
   * Resend all failed messages since a given time.
   *
   * Messages that were sent successfully, even if failed initially, are not resent.
   */
  public recover(
    appId: string,
    endpointId: string,
    recoverIn: RecoverIn,
    options?: EndpointRecoverOptions
  ): Promise<RecoverOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(RecoverInSerializer._toJsonObject(recoverIn));

    return request.send(this.requestCtx, RecoverOutSerializer._fromJsonObject);
  }

  /**
   * Replays messages to the endpoint.
   *
   * Only messages that were created after `since` will be sent.
   * Messages that were previously sent to the endpoint are not resent.
   */
  public replayMissing(
    appId: string,
    endpointId: string,
    replayIn: ReplayIn,
    options?: EndpointReplayMissingOptions
  ): Promise<ReplayOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ReplayInSerializer._toJsonObject(replayIn));

    return request.send(this.requestCtx, ReplayOutSerializer._fromJsonObject);
  }

  /**
   * Get the endpoint's signing secret.
   *
   * This is used to verify the authenticity of the webhook.
   * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
   */
  public getSecret(appId: string, endpointId: string): Promise<EndpointSecretOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, EndpointSecretOutSerializer._fromJsonObject);
  }

  /**
   * Rotates the endpoint's signing secret.
   *
   * The previous secret will remain valid for the next 24 hours.
   */
  public rotateSecret(
    appId: string,
    endpointId: string,
    endpointSecretRotateIn: EndpointSecretRotateIn,
    options?: EndpointRotateSecretOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      EndpointSecretRotateInSerializer._toJsonObject(endpointSecretRotateIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Send an example message for an event. */
  public sendExample(
    appId: string,
    endpointId: string,
    eventExampleIn: EventExampleIn,
    options?: EndpointSendExampleOptions
  ): Promise<MessageOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(EventExampleInSerializer._toJsonObject(eventExampleIn));

    return request.send(this.requestCtx, MessageOutSerializer._fromJsonObject);
  }

  /** Get basic statistics for the endpoint. */
  public getStats(
    appId: string,
    endpointId: string,
    options?: EndpointGetStatsOptions
  ): Promise<EndpointStats> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setQueryParam("since", options?.since);
    request.setQueryParam("until", options?.until);

    return request.send(this.requestCtx, EndpointStatsSerializer._fromJsonObject);
  }

  /** Get the transformation code associated with this endpoint. */
  public transformationGet(
    appId: string,
    endpointId: string
  ): Promise<EndpointTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      EndpointTransformationOutSerializer._fromJsonObject
    );
  }

  /** Set or unset the transformation code associated with this endpoint. */
  public transformationPartialUpdate(
    appId: string,
    endpointId: string,
    endpointTransformationIn: EndpointTransformationIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      EndpointTransformationInSerializer._toJsonObject(endpointTransformationIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
