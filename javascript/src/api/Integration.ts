// this file is @generated
import {
  IntegrationIn,
  IntegrationKeyOut,
  IntegrationOut,
  IntegrationUpdate,
  ListResponseIntegrationOut,
  Ordering,
} from "../openapi";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface IntegrationListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface IntegrationCreateOptions {
  idempotencyKey?: string;
}

export interface IntegrationRotateKeyOptions {
  idempotencyKey?: string;
}

export class Integration {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List the application's integrations. */
  public list(
    appId: string,
    options?: IntegrationListOptions
  ): Promise<ListResponseIntegrationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}/integration");

    request.setPathParam("app_id", appId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(this.requestCtx, "ListResponseIntegrationOut");
  }

  /** Create an integration. */
  public create(
    appId: string,
    integrationIn: IntegrationIn,
    options?: IntegrationCreateOptions
  ): Promise<IntegrationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app/{app_id}/integration");

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(integrationIn, "IntegrationIn");

    return request.send(this.requestCtx, "IntegrationOut");
  }

  /** Get an integration. */
  public get(appId: string, integId: string): Promise<IntegrationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/integration/{integ_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);

    return request.send(this.requestCtx, "IntegrationOut");
  }

  /** Update an integration. */
  public update(
    appId: string,
    integId: string,
    integrationUpdate: IntegrationUpdate
  ): Promise<IntegrationOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/integration/{integ_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);
    request.setBody(integrationUpdate, "IntegrationUpdate");

    return request.send(this.requestCtx, "IntegrationOut");
  }

  /** Delete an integration. */
  public delete(appId: string, integId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/integration/{integ_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * Get an integration's key.
   *
   * @deprecated
   */
  public getKey(appId: string, integId: string): Promise<IntegrationKeyOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/integration/{integ_id}/key"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);

    return request.send(this.requestCtx, "IntegrationKeyOut");
  }

  /** Rotate the integration's key. The previous key will be immediately revoked. */
  public rotateKey(
    appId: string,
    integId: string,
    options?: IntegrationRotateKeyOptions
  ): Promise<IntegrationKeyOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/integration/{integ_id}/key/rotate"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, "IntegrationKeyOut");
  }
}
