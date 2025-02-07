// this file is @generated
import {
  ListResponseOperationalWebhookEndpointOut,
  ListResponseOperationalWebhookEndpointOutUtil,
} from "../models/list_response_operational_webhook_endpoint_out";
import {
  OperationalWebhookEndpointHeadersIn,
  OperationalWebhookEndpointHeadersInUtil,
} from "../models/operational_webhook_endpoint_headers_in";
import {
  OperationalWebhookEndpointHeadersOut,
  OperationalWebhookEndpointHeadersOutUtil,
} from "../models/operational_webhook_endpoint_headers_out";
import {
  OperationalWebhookEndpointIn,
  OperationalWebhookEndpointInUtil,
} from "../models/operational_webhook_endpoint_in";
import {
  OperationalWebhookEndpointOut,
  OperationalWebhookEndpointOutUtil,
} from "../models/operational_webhook_endpoint_out";
import {
  OperationalWebhookEndpointSecretIn,
  OperationalWebhookEndpointSecretInUtil,
} from "../models/operational_webhook_endpoint_secret_in";
import {
  OperationalWebhookEndpointSecretOut,
  OperationalWebhookEndpointSecretOutUtil,
} from "../models/operational_webhook_endpoint_secret_out";
import {
  OperationalWebhookEndpointUpdate,
  OperationalWebhookEndpointUpdateUtil,
} from "../models/operational_webhook_endpoint_update";
import { Ordering, OrderingUtil } from "../models/ordering";
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

    return request.send(
      this.requestCtx,
      ListResponseOperationalWebhookEndpointOutUtil._fromJsonObject
    );
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
    request.setBody(
      OperationalWebhookEndpointInUtil._toJsonObject(operationalWebhookEndpointIn)
    );

    return request.send(
      this.requestCtx,
      OperationalWebhookEndpointOutUtil._fromJsonObject
    );
  }

  /** Get an operational webhook endpoint. */
  public get(endpointId: string): Promise<OperationalWebhookEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/operational-webhook/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      OperationalWebhookEndpointOutUtil._fromJsonObject
    );
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
    request.setBody(
      OperationalWebhookEndpointUpdateUtil._toJsonObject(operationalWebhookEndpointUpdate)
    );

    return request.send(
      this.requestCtx,
      OperationalWebhookEndpointOutUtil._fromJsonObject
    );
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

    return request.send(
      this.requestCtx,
      OperationalWebhookEndpointHeadersOutUtil._fromJsonObject
    );
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
      OperationalWebhookEndpointHeadersInUtil._toJsonObject(
        operationalWebhookEndpointHeadersIn
      )
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

    return request.send(
      this.requestCtx,
      OperationalWebhookEndpointSecretOutUtil._fromJsonObject
    );
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
      OperationalWebhookEndpointSecretInUtil._toJsonObject(
        operationalWebhookEndpointSecretIn
      )
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
