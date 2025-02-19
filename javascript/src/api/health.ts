// this file is @generated
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export class Health {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Verify the API server is up and running. */
  public get(): Promise<void> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/health");

    return request.sendNoResponseBody(this.requestCtx);
  }
}
