// this file is @generated
import { BackgroundTaskOut, BackgroundTaskOutUtil } from "../models/background_task_out";
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusUtil,
} from "../models/background_task_status";
import {
  BackgroundTaskType,
  BackgroundTaskTypeUtil,
} from "../models/background_task_type";
import {
  ListResponseBackgroundTaskOut,
  ListResponseBackgroundTaskOutUtil,
} from "../models/list_response_background_task_out";
import { Ordering, OrderingUtil } from "../models/ordering";
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

    return request.send(
      this.requestCtx,
      ListResponseBackgroundTaskOutUtil._fromJsonObject
    );
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

    return request.send(this.requestCtx, BackgroundTaskOutUtil._fromJsonObject);
  }
}
