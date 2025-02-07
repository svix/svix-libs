// this file is @generated
import {
  ApplicationIn,
  ApplicationOut,
  ApplicationPatch,
  ListResponseApplicationOut,
  Ordering,
} from "../openapi";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface ApplicationListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface ApplicationCreateOptions {
  idempotencyKey?: string;
}

export class Application {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the organization's applications. */
  public list(options?: ApplicationListOptions): Promise<ListResponseApplicationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(this.requestCtx, "ListResponseApplicationOut");
  }

  /** Create a new application. */
  public create(
    applicationIn: ApplicationIn,
    options?: ApplicationCreateOptions
  ): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(applicationIn, "ApplicationIn");

    return request.send(this.requestCtx, "ApplicationOut");
  }

  /** Get the application with the UID from `applicationIn`, or create it if it doesn't exist yet. */
  public getOrCreate(
    applicationIn: ApplicationIn,
    options?: ApplicationCreateOptions
  ): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app");

    request.setQueryParam("get_if_exists", true);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(applicationIn, "ApplicationIn");

    return request.send(this.requestCtx, "ApplicationOut");
  }

  /** Get an application. */
  public get(appId: string): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);

    return request.send(this.requestCtx, "ApplicationOut");
  }

  /** Update an application. */
  public update(appId: string, applicationIn: ApplicationIn): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);
    request.setBody(applicationIn, "ApplicationIn");

    return request.send(this.requestCtx, "ApplicationOut");
  }

  /** Delete an application. */
  public delete(appId: string): Promise<void> {
    const request = new SvixRequest(HttpMethod.DELETE, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update an application. */
  public patch(
    appId: string,
    applicationPatch: ApplicationPatch
  ): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.PATCH, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);
    request.setBody(applicationPatch, "ApplicationPatch");

    return request.send(this.requestCtx, "ApplicationOut");
  }
}
