// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.BackgroundTaskOut
import com.svix.kotlin.models.BackgroundTaskStatus
import com.svix.kotlin.models.BackgroundTaskType
import com.svix.kotlin.models.ListResponseBackgroundTaskOut
import com.svix.kotlin.models.Ordering
import okhttp3.HttpUrl

data class BackgroundTaskListOptions(
    val status: BackgroundTaskStatus? = null,
    val task: BackgroundTaskType? = null,
    val limit: ULong? = null,
    val iterator: String? = null,
    val order: Ordering? = null,
)

class BackgroundTask(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /** List background tasks executed in the past 90 days. */
    suspend fun list(
        options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ): ListResponseBackgroundTaskOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/background-task")
        options.status?.let { url = url.addQueryParameter("status", it.toString()) }
        options.task?.let { url = url.addQueryParameter("task", it.toString()) }
        options.limit?.let { url = url.addQueryParameter("limit", it.toString()) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.order?.let { url = url.addQueryParameter("order", it.toString()) }
        return this.executeRequest<Any, ListResponseBackgroundTaskOut>("GET", url.build())
    }

    /** Get a background task by ID. */
    suspend fun get(taskId: String): BackgroundTaskOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/background-task/$taskId")
        return this.executeRequest<Any, BackgroundTaskOut>("GET", url.build())
    }
}
