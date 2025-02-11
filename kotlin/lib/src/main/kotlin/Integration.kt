// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.IntegrationIn
import com.svix.kotlin.models.IntegrationKeyOut
import com.svix.kotlin.models.IntegrationOut
import com.svix.kotlin.models.IntegrationUpdate
import com.svix.kotlin.models.ListResponseIntegrationOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers
import okhttp3.HttpUrl

data class IntegrationListOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val order: Ordering? = null,
)

data class IntegrationCreateOptions(val idempotencyKey: String? = null)

data class IntegrationRotateKeyOptions(val idempotencyKey: String? = null)

class Integration(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /** List the application's integrations. */
    suspend fun list(
        appId: String,
        options: IntegrationListOptions = IntegrationListOptions(),
    ): ListResponseIntegrationOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration")
        options.limit?.let { url = url.addQueryParameter("limit", it.toString()) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.order?.let { url = url.addQueryParameter("order", it.toString()) }
        return this.executeRequest<Any, ListResponseIntegrationOut>("GET", url.build())
    }

    /** Create an integration. */
    suspend fun create(
        appId: String,
        integrationIn: IntegrationIn,
        options: IntegrationCreateOptions = IntegrationCreateOptions(),
    ): IntegrationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return this.executeRequest<IntegrationIn, IntegrationOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = integrationIn,
        )
    }

    /** Get an integration. */
    suspend fun get(appId: String, integId: String): IntegrationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId")
        return this.executeRequest<Any, IntegrationOut>("GET", url.build())
    }

    /** Update an integration. */
    suspend fun update(
        appId: String,
        integId: String,
        integrationUpdate: IntegrationUpdate,
    ): IntegrationOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId")

        return this.executeRequest<IntegrationUpdate, IntegrationOut>(
            "PUT",
            url.build(),
            reqBody = integrationUpdate,
        )
    }

    /** Delete an integration. */
    suspend fun delete(appId: String, integId: String) {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId")
        this.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /**
     * Get an integration's key.
     *
     * @deprecated
     */
    @Deprecated("")
    suspend fun getKey(appId: String, integId: String): IntegrationKeyOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId/key")
        return this.executeRequest<Any, IntegrationKeyOut>("GET", url.build())
    }

    /** Rotate the integration's key. The previous key will be immediately revoked. */
    suspend fun rotateKey(
        appId: String,
        integId: String,
        options: IntegrationRotateKeyOptions = IntegrationRotateKeyOptions(),
    ): IntegrationKeyOut {
        val url =
            this.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId/key/rotate")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }
        return this.executeRequest<Any, IntegrationKeyOut>(
            "POST",
            url.build(),
            headers = headers.build(),
        )
    }
}
