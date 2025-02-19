import "svix-fetch";
import { ApiException } from "./util";
import { HttpErrorOut, HTTPValidationError } from "./HttpErrors";

export const LIB_VERSION = "1.58.1";
const USER_AGENT = `svix-libs/${LIB_VERSION}/javascript`;

export enum HttpMethod {
  GET = "GET",
  HEAD = "HEAD",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
  CONNECT = "CONNECT",
  OPTIONS = "OPTIONS",
  TRACE = "TRACE",
  PATCH = "PATCH",
}

export interface SvixRequestContext {
  /** The API base URL, like "https://api.svix.com" */
  baseUrl: string;
  /** The 'bearer' scheme access token */
  token: string;
  /** Time in milliseconds to wait for requests to get a response. */
  timeout?: number;
}

type QueryParameter = string | boolean | number | Date | string[] | null | undefined;

export class SvixRequest {
  constructor(
    private readonly method: HttpMethod,
    private path: string
  ) {}

  private body?: string;
  private queryParams: Record<string, string> = {};
  private headerParams: Record<string, string> = {};

  public setPathParam(name: string, value: string) {
    const newPath = this.path.replace(`{${name}}`, encodeURIComponent(value));
    if (this.path === newPath) {
      throw new Error(`path parameter ${name} not found`);
    }
    this.path = newPath;
  }

  public setQueryParam(name: string, value: QueryParameter) {
    if (value === undefined || value === null) {
      return;
    }

    if (typeof value === "string") {
      this.queryParams[name] = value;
    } else if (typeof value === "boolean" || typeof value === "number") {
      this.queryParams[name] = value.toString();
    } else if (value instanceof Date) {
      this.queryParams[name] = value.toISOString();
    } else if (value instanceof Array) {
      if (value.length > 0) {
        this.queryParams[name] = value.join(",");
      }
    } else {
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const _assert_unreachable: never = value;
      throw new Error(`query parameter ${name} has unsupported type`);
    }
  }

  public setHeaderParam(name: string, value?: string) {
    if (value === undefined) {
      return;
    }

    this.headerParams[name] = value;
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public setBody(value: any) {
    this.body = JSON.stringify(value);
  }

  /**
   * Send this request, returning the request body as a caller-specified type.
   *
   * If the server returns a 422 error, an `ApiException<HTTPValidationError>` is thrown.
   * If the server returns another 4xx error, an `ApiException<HttpErrorOut>` is thrown.
   *
   * If the server returns a 5xx error, the request is retried up to two times with exponential backoff.
   * If retries are exhausted, an `ApiException<HttpErrorOut>` is thrown.
   */
  public async send<R>(
    ctx: SvixRequestContext,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    parseResponseBody: (jsonObject: any) => R
  ): Promise<R> {
    const responseBody = await this.sendInner(ctx);
    return parseResponseBody(JSON.parse(responseBody));
  }

  /** Same as `send`, but the response body is discarded, not parsed. */
  public async sendNoResponseBody(ctx: SvixRequestContext): Promise<void> {
    await this.sendInner(ctx);
  }

  private async sendInner(ctx: SvixRequestContext): Promise<string> {
    const url = new URL(ctx.baseUrl + this.path);
    for (const [name, value] of Object.entries(this.queryParams)) {
      url.searchParams.set(name, value);
    }

    const randomId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);

    // Cloudflare Workers fail if the credentials option is used in a fetch call.
    // This work around that. Source:
    // https://github.com/cloudflare/workers-sdk/issues/2514#issuecomment-2178070014
    const isCredentialsSupported = "credentials" in Request.prototype;

    const response = await sendWithRetry(url, {
      method: this.method.toString(),
      body: this.body,
      headers: {
        accept: "application/json, */*;q=0.8",
        authorization: `Bearer ${ctx.token}`,
        "user-agent": USER_AGENT,
        "svix-req-id": randomId.toString(),
        ...this.headerParams,
      },
      credentials: isCredentialsSupported ? "same-origin" : undefined,
      signal: ctx.timeout !== undefined ? AbortSignal.timeout(ctx.timeout) : undefined,
    });

    const responseBody = await response.text();

    if (response.status < 300) {
      // success case
      return responseBody;
    } else if (response.status === 422) {
      const body = JSON.parse(responseBody);
      throw new ApiException<HTTPValidationError>(
        response.status,
        body as HTTPValidationError,
        response.headers
      );
    } else if (response.status >= 400 && response.status <= 499) {
      const body = JSON.parse(responseBody);
      throw new ApiException<HttpErrorOut>(
        response.status,
        body as HttpErrorOut,
        response.headers
      );
    } else {
      throw new ApiException(response.status, responseBody, response.headers);
    }
  }
}

type SvixRequestInit = RequestInit & {
  headers: Record<string, string>;
};

async function sendWithRetry(
  url: URL,
  init: SvixRequestInit,
  triesLeft = 2,
  nextInterval = 50,
  retryCount = 1
): Promise<Response> {
  const sleep = (interval: number) =>
    new Promise((resolve) => setTimeout(resolve, interval));

  try {
    const response = await fetch(url, init);
    if (triesLeft <= 0 || response.status < 500) {
      return response;
    }
  } catch (e) {
    if (triesLeft <= 0) {
      throw e;
    }
  }

  await sleep(nextInterval);
  init.headers["svix-retry-count"] = retryCount.toString();
  return await sendWithRetry(url, init, --triesLeft, nextInterval * 2, ++retryCount);
}
