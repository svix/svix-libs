import { Authentication } from "./api/authentication";
import { Application } from "./api/application";
import { BackgroundTask } from "./api/backgroundTask";
import { Endpoint } from "./api/endpoint";
import { EventType } from "./api/eventType";
import { Integration } from "./api/integration";
import { Message } from "./api/message";
import { MessageAttempt } from "./api/messageAttempt";
import { OperationalWebhookEndpoint } from "./api/operationalWebhookEndpoint";
import { Statistics } from "./api/statistics";
import { SvixRequestContext } from "./request";

export { PostOptions } from "./util";
export * from "./webhook";

export { ApplicationListOptions } from "./api/application";
export { BackgroundTaskListOptions } from "./api/backgroundTask";
export { EndpointListOptions, EndpointGetStatsOptions } from "./api/endpoint";
export { EventTypeListOptions } from "./api/eventType";
export { IntegrationListOptions } from "./api/integration";
export { MessageListOptions, messageInRaw } from "./api/message";
export { MessageAttemptListByEndpointOptions } from "./api/messageAttempt";
export { OperationalWebhookEndpointListOptions } from "./api/operationalWebhookEndpoint";

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

export class Svix {
  private readonly requestCtx: SvixRequestContext;

  public constructor(token: string, options: SvixOptions = {}) {
    const regionalUrl = REGIONS.find((x) => x.region === token.split(".")[1])?.url;
    const baseUrl: string = options.serverUrl ?? regionalUrl ?? "https://api.svix.com";

    this.requestCtx = { baseUrl, token, timeout: options.requestTimeout };
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
