// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.BackgroundTaskOut
import com.svix.kotlin.models.BackgroundTaskStatus
import com.svix.kotlin.models.BackgroundTaskType
import com.svix.kotlin.models.ListResponseBackgroundTaskOut
import com.svix.kotlin.models.Ordering

data class BackgroundTaskListOptions(
    val status: BackgroundTaskStatus? = null,
    val task: BackgroundTaskType? = null,
    val limit: ULong? = null,
    val iterator: String? = null,
    val order: Ordering? = null,
)

class BackgroundTask(private val client: SvixHttpClient) {

    /** List background tasks executed in the past 90 days. */
    suspend fun list(
        options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ): ListResponseBackgroundTaskOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/background-task")
        options.status?.let { url.addQueryParameter("status", serializeQueryParam(it)) }
        options.task?.let { url.addQueryParameter("task", serializeQueryParam(it)) }
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseBackgroundTaskOut>("GET", url.build())
    }

    /** Get a background task by ID. */
    suspend fun get(taskId: String): BackgroundTaskOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/background-task/$taskId")
        return client.executeRequest<Any, BackgroundTaskOut>("GET", url.build())
    }
}
