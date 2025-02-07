// this file is @generated
import {
  ListResponseOperationalWebhookEndpointOut,
  OperationalWebhookEndpointHeadersIn,
  OperationalWebhookEndpointHeadersOut,
  OperationalWebhookEndpointIn,
  OperationalWebhookEndpointOut,
  OperationalWebhookEndpointSecretIn,
  OperationalWebhookEndpointSecretOut,
  OperationalWebhookEndpointUpdate,
  Ordering,
} from "../openapi";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface OperationalWebhookEndpointListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface OperationalWebhookEndpointCreateOptions {
  idempotencyKey?: string;
}

export interface OperationalWebhookEndpointRotateSecretOptions {
  idempotencyKey?: string;
}

export class OperationalWebhookEndpoint {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List operational webhook endpoints. */
  public list(
    options?: OperationalWebhookEndpointListOptions
  ): Promise<ListResponseOperationalWebhookEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/operational-webhook/endpoint"
    );

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(this.requestCtx, "ListResponseOperationalWebhookEndpointOut");
  }

  /** Create an operational webhook endpoint. */
  public create(
    operationalWebhookEndpointIn: OperationalWebhookEndpointIn,
    options?: OperationalWebhookEndpointCreateOptions
  ): Promise<OperationalWebhookEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/operational-webhook/endpoint"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(operationalWebhookEndpointIn, "OperationalWebhookEndpointIn");

    return request.send(this.requestCtx, "OperationalWebhookEndpointOut");
  }

  /** Get an operational webhook endpoint. */
  public get(endpointId: string): Promise<OperationalWebhookEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, "OperationalWebhookEndpointOut");
  }

  /** Update an operational webhook endpoint. */
  public update(
    endpointId: string,
    operationalWebhookEndpointUpdate: OperationalWebhookEndpointUpdate
  ): Promise<OperationalWebhookEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);
    request.setBody(operationalWebhookEndpointUpdate, "OperationalWebhookEndpointUpdate");

    return request.send(this.requestCtx, "OperationalWebhookEndpointOut");
  }

  /** Delete an operational webhook endpoint. */
  public delete(endpointId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Get the additional headers to be sent with the operational webhook. */
  public getHeaders(endpointId: string): Promise<OperationalWebhookEndpointHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, "OperationalWebhookEndpointHeadersOut");
  }

  /** Set the additional headers to be sent with the operational webhook. */
  public updateHeaders(
    endpointId: string,
    operationalWebhookEndpointHeadersIn: OperationalWebhookEndpointHeadersIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      operationalWebhookEndpointHeadersIn,
      "OperationalWebhookEndpointHeadersIn"
    );

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * Get an operational webhook endpoint's signing secret.
   *
   * This is used to verify the authenticity of the webhook.
   * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
   */
  public getSecret(endpointId: string): Promise<OperationalWebhookEndpointSecretOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, "OperationalWebhookEndpointSecretOut");
  }

  /**
   * Rotates an operational webhook endpoint's signing secret.
   *
   * The previous secret will remain valid for the next 24 hours.
   */
  public rotateSecret(
    endpointId: string,
    operationalWebhookEndpointSecretIn: OperationalWebhookEndpointSecretIn,
    options?: OperationalWebhookEndpointRotateSecretOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate"
    );

    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      operationalWebhookEndpointSecretIn,
      "OperationalWebhookEndpointSecretIn"
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
