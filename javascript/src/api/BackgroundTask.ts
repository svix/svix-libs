// this file is @generated
import {
  BackgroundTaskOut,
  BackgroundTaskStatus,
  BackgroundTaskType,
  ListResponseBackgroundTaskOut,
  Ordering,
} from "../openapi";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface BackgroundTaskListOptions {
  /** Filter the response based on the status. */
  status?: BackgroundTaskStatus;
  /** Filter the response based on the type. */
  task?: BackgroundTaskType;
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export class BackgroundTask {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List background tasks executed in the past 90 days. */
  public list(
    options?: BackgroundTaskListOptions
  ): Promise<ListResponseBackgroundTaskOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/background-task");

    request.setQueryParam("status", options?.status);
    request.setQueryParam("task", options?.task);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(this.requestCtx, "ListResponseBackgroundTaskOut");
  }

  /**
   * List background tasks executed in the past 90 days.
   *
   * @deprecated Use list instead.
   * */
  public listByEndpoint(
    options?: BackgroundTaskListOptions
  ): Promise<ListResponseBackgroundTaskOut> {
    return this.list(options);
  }

  /** Get a background task by ID. */
  public get(taskId: string): Promise<BackgroundTaskOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/background-task/{task_id}");

    request.setPathParam("task_id", taskId);

    return request.send(this.requestCtx, "BackgroundTaskOut");
  }
}
