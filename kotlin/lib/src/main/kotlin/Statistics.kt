// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.AggregateEventTypesOut
import com.svix.kotlin.models.AppUsageStatsIn
import com.svix.kotlin.models.AppUsageStatsOut
import okhttp3.Headers
import okhttp3.HttpUrl

data class StatisticsAggregateAppStatsOptions(val idempotencyKey: String? = null)

class Statistics(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /**
     * Creates a background task to calculate the message destinations for all applications in the
     * environment.
     *
     * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task`
     * endpoint to retrieve the results of the operation.
     */
    suspend fun aggregateAppStats(
        appUsageStatsIn: AppUsageStatsIn,
        options: StatisticsAggregateAppStatsOptions = StatisticsAggregateAppStatsOptions(),
    ): AppUsageStatsOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/stats/usage/app")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return this.executeRequest<AppUsageStatsIn, AppUsageStatsOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = appUsageStatsIn,
        )
    }

    /**
     * Creates a background task to calculate the listed event types for all apps in the
     * organization.
     *
     * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task`
     * endpoint to retrieve the results of the operation.
     */
    suspend fun aggregateEventTypes(): AggregateEventTypesOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/stats/usage/event-types")
        return this.executeRequest<Any, AggregateEventTypesOut>("PUT", url.build())
    }
}
