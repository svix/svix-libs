// this file is @generated
import {
  AggregateEventTypesOut,
  AggregateEventTypesOutSerializer,
} from "../models/aggregateEventTypesOut";
import { AppUsageStatsIn, AppUsageStatsInSerializer } from "../models/appUsageStatsIn";
import { AppUsageStatsOut, AppUsageStatsOutSerializer } from "../models/appUsageStatsOut";
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
    request.setBody(AppUsageStatsInSerializer._toJsonObject(appUsageStatsIn));

    return request.send(this.requestCtx, AppUsageStatsOutSerializer._fromJsonObject);
  }

  /**
   * Creates a background task to calculate the listed event types for all apps in the organization.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   */
  public aggregateEventTypes(): Promise<AggregateEventTypesOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/stats/usage/event-types");

    return request.send(
      this.requestCtx,
      AggregateEventTypesOutSerializer._fromJsonObject
    );
  }
}
