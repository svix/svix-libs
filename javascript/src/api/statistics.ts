// this file is @generated
import {
  AggregateEventTypesOut,
  AggregateEventTypesOutUtil,
} from "../models/aggregate_event_types_out";
import { AppUsageStatsIn, AppUsageStatsInUtil } from "../models/app_usage_stats_in";
import { AppUsageStatsOut, AppUsageStatsOutUtil } from "../models/app_usage_stats_out";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface StatisticsAggregateAppStatsOptions {
  idempotencyKey?: string;
}

export class Statistics {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /**
   * Creates a background task to calculate the message destinations for all applications in the environment.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   */
  public aggregateAppStats(
    appUsageStatsIn: AppUsageStatsIn,
    options?: StatisticsAggregateAppStatsOptions
  ): Promise<AppUsageStatsOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stats/usage/app");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(AppUsageStatsInUtil._toJsonObject(appUsageStatsIn));

    return request.send(this.requestCtx, AppUsageStatsOutUtil._fromJsonObject);
  }

  /**
   * Creates a background task to calculate the listed event types for all apps in the organization.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   */
  public aggregateEventTypes(): Promise<AggregateEventTypesOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/stats/usage/event-types");

    return request.send(this.requestCtx, AggregateEventTypesOutUtil._fromJsonObject);
  }
}
