// this file is @generated
import { EnvironmentIn, EnvironmentInSerializer } from "../models/environmentIn";
import { EnvironmentOut, EnvironmentOutSerializer } from "../models/environmentOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface EnvironmentExportOptions {
  idempotencyKey?: string;
}

export interface EnvironmentImportOptions {
  idempotencyKey?: string;
}

export class Environment {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Download a JSON file containing all org-settings and event types. */
  public export(options?: EnvironmentExportOptions): Promise<EnvironmentOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/environment/export");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, EnvironmentOutSerializer._fromJsonObject);
  }

  /**
   * Import a configuration into the active organization.
   * It doesn't delete anything, only adds/updates what was passed to it.
   */
  public import(
    environmentIn: EnvironmentIn,
    options?: EnvironmentImportOptions
  ): Promise<void> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/environment/import");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(EnvironmentInSerializer._toJsonObject(environmentIn));

    return request.sendNoResponseBody(this.requestCtx);
  }
}
