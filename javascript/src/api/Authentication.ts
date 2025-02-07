// this file is @generated
import {
  AppPortalAccessIn,
  AppPortalAccessOut,
  ApplicationTokenExpireIn,
  DashboardAccessOut,
} from "../openapi";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface AuthenticationAppPortalAccessOptions {
  idempotencyKey?: string;
}

export interface AuthenticationExpireAllOptions {
  idempotencyKey?: string;
}

export interface AuthenticationDashboardAccessOptions {
  idempotencyKey?: string;
}

export interface AuthenticationLogoutOptions {
  idempotencyKey?: string;
}

export class Authentication {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal. */
  public appPortalAccess(
    appId: string,
    appPortalAccessIn: AppPortalAccessIn,
    options?: AuthenticationAppPortalAccessOptions
  ): Promise<AppPortalAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app-portal-access/{app_id}"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(appPortalAccessIn, "AppPortalAccessIn");

    return request.send(this.requestCtx, "AppPortalAccessOut");
  }

  /** Expire all of the tokens associated with a specific application. */
  public expireAll(
    appId: string,
    applicationTokenExpireIn: ApplicationTokenExpireIn,
    options?: AuthenticationExpireAllOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app/{app_id}/expire-all"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(applicationTokenExpireIn, "ApplicationTokenExpireIn");

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * DEPRECATED: Please use `app-portal-access` instead.
   *
   * Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
   *
   * @deprecated
   */
  public dashboardAccess(
    appId: string,
    options?: AuthenticationDashboardAccessOptions
  ): Promise<DashboardAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/dashboard-access/{app_id}"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, "DashboardAccessOut");
  }

  /**
   * Logout an app token.
   *
   * Trying to log out other tokens will fail.
   */
  public logout(options?: AuthenticationLogoutOptions): Promise<void> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/auth/logout");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.sendNoResponseBody(this.requestCtx);
  }
}
