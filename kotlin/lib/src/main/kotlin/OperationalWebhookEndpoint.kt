// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ListResponseOperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointHeadersIn
import com.svix.kotlin.models.OperationalWebhookEndpointHeadersOut
import com.svix.kotlin.models.OperationalWebhookEndpointIn
import com.svix.kotlin.models.OperationalWebhookEndpointOut
import com.svix.kotlin.models.OperationalWebhookEndpointSecretIn
import com.svix.kotlin.models.OperationalWebhookEndpointSecretOut
import com.svix.kotlin.models.OperationalWebhookEndpointUpdate
import com.svix.kotlin.models.Ordering
import okhttp3.Headers
import okhttp3.HttpUrl

data class OperationalWebhookEndpointListOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val order: Ordering? = null,
)

data class OperationalWebhookEndpointCreateOptions(val idempotencyKey: String? = null)

data class OperationalWebhookEndpointRotateSecretOptions(val idempotencyKey: String? = null)

class OperationalWebhookEndpoint(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /** List operational webhook endpoints. */
    suspend fun list(
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions()
    ): ListResponseOperationalWebhookEndpointOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint")
        options.limit?.let { url = url.addQueryParameter("limit", it.toString()) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.order?.let { url = url.addQueryParameter("order", it.toString()) }
        return this.executeRequest<Any, ListResponseOperationalWebhookEndpointOut>(
            "GET",
            url.build(),
        )
    }

    /** Create an operational webhook endpoint. */
    suspend fun create(
        operationalWebhookEndpointIn: OperationalWebhookEndpointIn,
        options: OperationalWebhookEndpointCreateOptions = OperationalWebhookEndpointCreateOptions(),
    ): OperationalWebhookEndpointOut {
        val url = this.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return this.executeRequest<OperationalWebhookEndpointIn, OperationalWebhookEndpointOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = operationalWebhookEndpointIn,
        )
    }

    /** Get an operational webhook endpoint. */
    suspend fun get(endpointId: String): OperationalWebhookEndpointOut {
        val url =
            this.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint/$endpointId")
        return this.executeRequest<Any, OperationalWebhookEndpointOut>("GET", url.build())
    }

    /** Update an operational webhook endpoint. */
    suspend fun update(
        endpointId: String,
        operationalWebhookEndpointUpdate: OperationalWebhookEndpointUpdate,
    ): OperationalWebhookEndpointOut {
        val url =
            this.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint/$endpointId")

        return this.executeRequest<OperationalWebhookEndpointUpdate, OperationalWebhookEndpointOut>(
            "PUT",
            url.build(),
            reqBody = operationalWebhookEndpointUpdate,
        )
    }

    /** Delete an operational webhook endpoint. */
    suspend fun delete(endpointId: String) {
        val url =
            this.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint/$endpointId")
        this.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Get the additional headers to be sent with the operational webhook. */
    suspend fun getHeaders(endpointId: String): OperationalWebhookEndpointHeadersOut {
        val url =
            this.newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/headers")
        return this.executeRequest<Any, OperationalWebhookEndpointHeadersOut>("GET", url.build())
    }

    /** Set the additional headers to be sent with the operational webhook. */
    suspend fun updateHeaders(
        endpointId: String,
        operationalWebhookEndpointHeadersIn: OperationalWebhookEndpointHeadersIn,
    ) {
        val url =
            this.newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/headers")

        this.executeRequest<OperationalWebhookEndpointHeadersIn, Boolean>(
            "PUT",
            url.build(),
            reqBody = operationalWebhookEndpointHeadersIn,
        )
    }

    /**
     * Get an operational webhook endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook. For more information please refer to
     * [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    suspend fun getSecret(endpointId: String): OperationalWebhookEndpointSecretOut {
        val url =
            this.newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/secret")
        return this.executeRequest<Any, OperationalWebhookEndpointSecretOut>("GET", url.build())
    }

    /**
     * Rotates an operational webhook endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    suspend fun rotateSecret(
        endpointId: String,
        operationalWebhookEndpointSecretIn: OperationalWebhookEndpointSecretIn,
        options: OperationalWebhookEndpointRotateSecretOptions =
            OperationalWebhookEndpointRotateSecretOptions(),
    ) {
        val url =
            this.newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/secret/rotate")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        this.executeRequest<OperationalWebhookEndpointSecretIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = operationalWebhookEndpointSecretIn,
        )
    }
}
