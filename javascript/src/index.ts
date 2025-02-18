import { Authentication } from "./api/Authentication";
import { Application } from "./api/Application";
import { BackgroundTask } from "./api/BackgroundTask";
import { Endpoint } from "./api/Endpoint";
import { EventType } from "./api/EventType";
import { Integration } from "./api/Integration";
import { Message } from "./api/message";
import { MessageAttempt } from "./api/MessageAttempt";
import { OperationalWebhookEndpoint } from "./api/OperationalWebhookEndpoint";
import { Statistics } from "./api/Statistics";
import { LIB_VERSION, SvixRequestContext } from "./request";
import * as openapi from "./openapi";

export * from "./openapi/models/all";
export * from "./openapi/apis/exception";

export { PostOptions } from "./util";
export * from "./webhook";

export { ApplicationListOptions } from "./api/Application";
export { BackgroundTaskListOptions } from "./api/BackgroundTask";
export { EndpointListOptions, EndpointGetStatsOptions } from "./api/Endpoint";
export { EventTypeListOptions } from "./api/EventType";
export { IntegrationListOptions } from "./api/Integration";
export { MessageListOptions, messageInRaw } from "./api/message";
export {
  MessageAttemptListByEndpointOptions,
  MessageAttemptListOptions,
} from "./api/MessageAttempt";
export { OperationalWebhookEndpointListOptions } from "./api/OperationalWebhookEndpoint";

export interface SvixOptions {
  debug?: boolean;
  serverUrl?: string;
  /** Time in milliseconds to wait for requests to get a response. */
  requestTimeout?: number;
}

const REGIONS = [
  { region: "us", url: "https://api.us.svix.com" },
  { region: "eu", url: "https://api.eu.svix.com" },
  { region: "in", url: "https://api.in.svix.com" },
];

class UserAgentMiddleware implements openapi.Middleware {
  public pre(context: openapi.RequestContext): Promise<openapi.RequestContext> {
    context.setHeaderParam("User-Agent", `svix-libs/${LIB_VERSION}/javascript`);
    return Promise.resolve(context);
  }

  public post(context: openapi.ResponseContext): Promise<openapi.ResponseContext> {
    return Promise.resolve(context);
  }
}

export class Svix {
  private readonly requestCtx: SvixRequestContext;
  public readonly _configuration: openapi.Configuration;

  public constructor(token: string, options: SvixOptions = {}) {
    const regionalUrl = REGIONS.find((x) => x.region === token.split(".")[1])?.url;
    const baseUrl: string = options.serverUrl ?? regionalUrl ?? "https://api.svix.com";

    this.requestCtx = { baseUrl, token, timeout: options.requestTimeout };

    this._configuration = openapi.createConfiguration({
      baseServer: new openapi.ServerConfiguration<any>(baseUrl, {}),
      promiseMiddleware: [new UserAgentMiddleware()],
      authMethods: {
        HTTPBearer: {
          tokenProvider: {
            getToken: () => token,
          },
        },
      },
    });
  }

  public get authentication() {
    return new Authentication(this.requestCtx);
  }

  public get application() {
    return new Application(this.requestCtx);
  }

  public get endpoint() {
    return new Endpoint(this.requestCtx);
  }

  public get eventType() {
    return new EventType(this.requestCtx);
  }

  public get integration() {
    return new Integration(this.requestCtx);
  }

  public get message() {
    return new Message(this.requestCtx);
  }

  public get messageAttempt() {
    return new MessageAttempt(this.requestCtx);
  }

  public get backgroundTask() {
    return new BackgroundTask(this.requestCtx);
  }

  public get statistics() {
    return new Statistics(this.requestCtx);
  }

  public get operationalWebhookEndpoint() {
    return new OperationalWebhookEndpoint(this.requestCtx);
  }
}
