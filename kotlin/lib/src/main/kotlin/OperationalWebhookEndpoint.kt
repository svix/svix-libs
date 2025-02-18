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

data class OperationalWebhookEndpointListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class OperationalWebhookEndpointCreateOptions(val idempotencyKey: String? = null)

data class OperationalWebhookEndpointRotateSecretOptions(val idempotencyKey: String? = null)

class OperationalWebhookEndpoint(private val client: SvixHttpClient) {

    /** List operational webhook endpoints. */
    suspend fun list(
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions()
    ): ListResponseOperationalWebhookEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseOperationalWebhookEndpointOut>(
            "GET",
            url.build(),
        )
    }

    /** Create an operational webhook endpoint. */
    suspend fun create(
        operationalWebhookEndpointIn: OperationalWebhookEndpointIn,
        options: OperationalWebhookEndpointCreateOptions = OperationalWebhookEndpointCreateOptions(),
    ): OperationalWebhookEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<OperationalWebhookEndpointIn, OperationalWebhookEndpointOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = operationalWebhookEndpointIn,
        )
    }

    /** Get an operational webhook endpoint. */
    suspend fun get(endpointId: String): OperationalWebhookEndpointOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint/$endpointId")
        return client.executeRequest<Any, OperationalWebhookEndpointOut>("GET", url.build())
    }

    /** Update an operational webhook endpoint. */
    suspend fun update(
        endpointId: String,
        operationalWebhookEndpointUpdate: OperationalWebhookEndpointUpdate,
    ): OperationalWebhookEndpointOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint/$endpointId")

        return client.executeRequest<
            OperationalWebhookEndpointUpdate,
            OperationalWebhookEndpointOut,
        >(
            "PUT",
            url.build(),
            reqBody = operationalWebhookEndpointUpdate,
        )
    }

    /** Delete an operational webhook endpoint. */
    suspend fun delete(endpointId: String) {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint/$endpointId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Get the additional headers to be sent with the operational webhook. */
    suspend fun getHeaders(endpointId: String): OperationalWebhookEndpointHeadersOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/headers")
        return client.executeRequest<Any, OperationalWebhookEndpointHeadersOut>("GET", url.build())
    }

    /** Set the additional headers to be sent with the operational webhook. */
    suspend fun updateHeaders(
        endpointId: String,
        operationalWebhookEndpointHeadersIn: OperationalWebhookEndpointHeadersIn,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/headers")

        client.executeRequest<OperationalWebhookEndpointHeadersIn, Boolean>(
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
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/secret")
        return client.executeRequest<Any, OperationalWebhookEndpointSecretOut>("GET", url.build())
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
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/operational-webhook/endpoint/$endpointId/secret/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<OperationalWebhookEndpointSecretIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = operationalWebhookEndpointSecretIn,
        )
    }
}
