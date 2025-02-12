// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.ApplicationOut
import com.svix.kotlin.models.ApplicationPatch
import com.svix.kotlin.models.ListResponseApplicationOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers
import okhttp3.HttpUrl

data class ApplicationListOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val order: Ordering? = null,
)

data class ApplicationCreateOptions(val idempotencyKey: String? = null)

class Application(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /** List of all the organization's applications. */
    suspend fun list(
        options: ApplicationListOptions = ApplicationListOptions()
    ): ListResponseApplicationOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app")
        options.limit?.let { url = url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.order?.let { url = url.addQueryParameter("order", serializeQueryParam(it)) }
        return this.executeRequest<Any, ListResponseApplicationOut>("GET", url.build())
    }

    /** Create a new application. */
    suspend fun create(
        applicationIn: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ): ApplicationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return this.executeRequest<ApplicationIn, ApplicationOut>(
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
            this.newUrlBuilder()
                .encodedPath("/api/v1/app")
                .addQueryParameter("get_if_exists", "true")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return this.executeRequest<ApplicationIn, ApplicationOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = applicationIn,
        )
    }

    /** Get an application. */
    suspend fun get(appId: String): ApplicationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId")
        return this.executeRequest<Any, ApplicationOut>("GET", url.build())
    }

    /** Update an application. */
    suspend fun update(appId: String, applicationIn: ApplicationIn): ApplicationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId")

        return this.executeRequest<ApplicationIn, ApplicationOut>(
            "PUT",
            url.build(),
            reqBody = applicationIn,
        )
    }

    /** Delete an application. */
    suspend fun delete(appId: String) {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId")
        this.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update an application. */
    suspend fun patch(appId: String, applicationPatch: ApplicationPatch): ApplicationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId")

        return this.executeRequest<ApplicationPatch, ApplicationOut>(
            "PATCH",
            url.build(),
            reqBody = applicationPatch,
        )
    }
}
