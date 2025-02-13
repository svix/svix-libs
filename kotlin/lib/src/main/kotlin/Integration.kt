// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.IntegrationIn
import com.svix.kotlin.models.IntegrationKeyOut
import com.svix.kotlin.models.IntegrationOut
import com.svix.kotlin.models.IntegrationUpdate
import com.svix.kotlin.models.ListResponseIntegrationOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers

data class IntegrationListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class IntegrationCreateOptions(val idempotencyKey: String? = null)

data class IntegrationRotateKeyOptions(val idempotencyKey: String? = null)

class Integration(private val client: SvixHttpClient) {

    /** List the application's integrations. */
    suspend fun list(
        appId: String,
        options: IntegrationListOptions = IntegrationListOptions(),
    ): ListResponseIntegrationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseIntegrationOut>("GET", url.build())
    }

    /** Create an integration. */
    suspend fun create(
        appId: String,
        integrationIn: IntegrationIn,
        options: IntegrationCreateOptions = IntegrationCreateOptions(),
    ): IntegrationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<IntegrationIn, IntegrationOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = integrationIn,
        )
    }

    /** Get an integration. */
    suspend fun get(appId: String, integId: String): IntegrationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId")
        return client.executeRequest<Any, IntegrationOut>("GET", url.build())
    }

    /** Update an integration. */
    suspend fun update(
        appId: String,
        integId: String,
        integrationUpdate: IntegrationUpdate,
    ): IntegrationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId")

        return client.executeRequest<IntegrationUpdate, IntegrationOut>(
            "PUT",
            url.build(),
            reqBody = integrationUpdate,
        )
    }

    /** Delete an integration. */
    suspend fun delete(appId: String, integId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /**
     * Get an integration's key.
     *
     * @deprecated
     */
    @Deprecated("")
    suspend fun getKey(appId: String, integId: String): IntegrationKeyOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId/key")
        return client.executeRequest<Any, IntegrationKeyOut>("GET", url.build())
    }

    /** Rotate the integration's key. The previous key will be immediately revoked. */
    suspend fun rotateKey(
        appId: String,
        integId: String,
        options: IntegrationRotateKeyOptions = IntegrationRotateKeyOptions(),
    ): IntegrationKeyOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/integration/$integId/key/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }
        return client.executeRequest<Any, IntegrationKeyOut>(
            "POST",
            url.build(),
            headers = headers.build(),
        )
    }
}
