// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ApplicationPatch
import com.svix.kotlin.models.ListResponseApplicationOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers

data class ApplicationListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class ApplicationCreateOptions(val idempotencyKey: String? = null)

class Application(private val client: SvixHttpClient) {

    /** List of all the organization's applications. */
    suspend fun list(
        options: ApplicationListOptions = ApplicationListOptions()
    ): ListResponseApplicationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseApplicationOut>("GET", url.build())
    }

    /** Create a new application. */
    suspend fun create(
        applicationIn: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ): ApplicationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<ApplicationIn, ApplicationOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = applicationIn,
        )
    }

    /** Get or create an application. */
    suspend fun getOrCreate(
        applicationIn: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ): ApplicationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app")
                .addQueryParameter("get_if_exists", "true")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return client.executeRequest<ApplicationIn, ApplicationOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = applicationIn,
        )
    }

    /** Get an application. */
    suspend fun get(appId: String): ApplicationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId")
        return client.executeRequest<Any, ApplicationOut>("GET", url.build())
    }

    /** Update an application. */
    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId")

        return client.executeRequest<ApplicationIn, ApplicationOut>(
            "PUT",
            url.build(),
            reqBody = applicationIn,
        )
    }

    /** Delete an application. */
    suspend fun delete(appId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update an application. */
    suspend fun patch(appId: String, applicationPatch: ApplicationPatch): ApplicationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId")

        return client.executeRequest<ApplicationPatch, ApplicationOut>(
            "PATCH",
            url.build(),
            reqBody = applicationPatch,
        )
    }
}
