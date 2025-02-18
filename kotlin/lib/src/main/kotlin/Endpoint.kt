// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EndpointHeadersIn
import com.svix.kotlin.models.EndpointHeadersOut
import com.svix.kotlin.models.EndpointHeadersPatchIn
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.EndpointPatch
import com.svix.kotlin.models.EndpointSecretOut
import com.svix.kotlin.models.EndpointSecretRotateIn
import com.svix.kotlin.models.EndpointStats
import com.svix.kotlin.models.EndpointTransformationIn
import com.svix.kotlin.models.EndpointTransformationOut
import com.svix.kotlin.models.EndpointUpdate
import com.svix.kotlin.models.EventExampleIn
import com.svix.kotlin.models.ListResponseEndpointOut
import com.svix.kotlin.models.MessageOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.models.RecoverIn
import com.svix.kotlin.models.RecoverOut
import com.svix.kotlin.models.ReplayIn
import com.svix.kotlin.models.ReplayOut
import kotlinx.datetime.Instant
import okhttp3.Headers

data class EndpointListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class EndpointCreateOptions(val idempotencyKey: String? = null)

data class EndpointRecoverOptions(val idempotencyKey: String? = null)

data class EndpointReplayMissingOptions(val idempotencyKey: String? = null)

data class EndpointRotateSecretOptions(val idempotencyKey: String? = null)

data class EndpointSendExampleOptions(val idempotencyKey: String? = null)

data class EndpointGetStatsOptions(
    /** Filter the range to data starting from this date. */
    val since: Instant? = null,
    /** Filter the range to data ending by this date. */
    val until: Instant? = null,
)

class Endpoint(private val client: SvixHttpClient) {

    /** List the application's endpoints. */
    suspend fun list(
        appId: String,
        options: EndpointListOptions = EndpointListOptions(),
    ): ListResponseEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseEndpointOut>("GET", url.build())
    }

    /**
     * Create a new endpoint for the application.
     *
     * When `secret` is `null` the secret is automatically generated (recommended).
     */
    suspend fun create(
        appId: String,
        endpointIn: EndpointIn,
        options: EndpointCreateOptions = EndpointCreateOptions(),
    ): EndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<EndpointIn, EndpointOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = endpointIn,
        )
    }

    /** Get an endpoint. */
    suspend fun get(appId: String, endpointId: String): EndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId")
        return client.executeRequest<Any, EndpointOut>("GET", url.build())
    }

    /** Update an endpoint. */
    suspend fun update(
        appId: String,
        endpointId: String,
        endpointUpdate: EndpointUpdate,
    ): EndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId")

        return client.executeRequest<EndpointUpdate, EndpointOut>(
            "PUT",
            url.build(),
            reqBody = endpointUpdate,
        )
    }

    /** Delete an endpoint. */
    suspend fun delete(appId: String, endpointId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update an endpoint. */
    suspend fun patch(
        appId: String,
        endpointId: String,
        endpointPatch: EndpointPatch,
    ): EndpointOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId")

        return client.executeRequest<EndpointPatch, EndpointOut>(
            "PATCH",
            url.build(),
            reqBody = endpointPatch,
        )
    }

    /** Get the additional headers to be sent with the webhook. */
    suspend fun getHeaders(appId: String, endpointId: String): EndpointHeadersOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/headers")
        return client.executeRequest<Any, EndpointHeadersOut>("GET", url.build())
    }

    /** Set the additional headers to be sent with the webhook. */
    suspend fun updateHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersIn: EndpointHeadersIn,
    ) {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/headers")

        client.executeRequest<EndpointHeadersIn, Boolean>(
            "PUT",
            url.build(),
            reqBody = endpointHeadersIn,
        )
    }

    /** Partially set the additional headers to be sent with the webhook. */
    suspend fun patchHeaders(
        appId: String,
        endpointId: String,
        endpointHeadersPatchIn: EndpointHeadersPatchIn,
    ) {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/headers")

        client.executeRequest<EndpointHeadersPatchIn, Boolean>(
            "PATCH",
            url.build(),
            reqBody = endpointHeadersPatchIn,
        )
    }

    /**
     * Resend all failed messages since a given time.
     *
     * Messages that were sent successfully, even if failed initially, are not resent.
     */
    suspend fun recover(
        appId: String,
        endpointId: String,
        recoverIn: RecoverIn,
        options: EndpointRecoverOptions = EndpointRecoverOptions(),
    ): RecoverOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/recover")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<RecoverIn, RecoverOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = recoverIn,
        )
    }

    /**
     * Replays messages to the endpoint.
     *
     * Only messages that were created after `since` will be sent. Messages that were previously
     * sent to the endpoint are not resent.
     */
    suspend fun replayMissing(
        appId: String,
        endpointId: String,
        replayIn: ReplayIn,
        options: EndpointReplayMissingOptions = EndpointReplayMissingOptions(),
    ): ReplayOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/replay-missing")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<ReplayIn, ReplayOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = replayIn,
        )
    }

    /**
     * Get the endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook. For more information please refer to
     * [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    suspend fun getSecret(appId: String, endpointId: String): EndpointSecretOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/secret")
        return client.executeRequest<Any, EndpointSecretOut>("GET", url.build())
    }

    /**
     * Rotates the endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    suspend fun rotateSecret(
        appId: String,
        endpointId: String,
        endpointSecretRotateIn: EndpointSecretRotateIn,
        options: EndpointRotateSecretOptions = EndpointRotateSecretOptions(),
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/secret/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<EndpointSecretRotateIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = endpointSecretRotateIn,
        )
    }

    /** Send an example message for an event. */
    suspend fun sendExample(
        appId: String,
        endpointId: String,
        eventExampleIn: EventExampleIn,
        options: EndpointSendExampleOptions = EndpointSendExampleOptions(),
    ): MessageOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/send-example")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<EventExampleIn, MessageOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = eventExampleIn,
        )
    }

    /** Get basic statistics for the endpoint. */
    suspend fun getStats(
        appId: String,
        endpointId: String,
        options: EndpointGetStatsOptions = EndpointGetStatsOptions(),
    ): EndpointStats {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/stats")
        options.since?.let { url.addQueryParameter("since", serializeQueryParam(it)) }
        options.until?.let { url.addQueryParameter("until", serializeQueryParam(it)) }
        return client.executeRequest<Any, EndpointStats>("GET", url.build())
    }

    /** Get the transformation code associated with this endpoint. */
    suspend fun transformationGet(appId: String, endpointId: String): EndpointTransformationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/transformation")
        return client.executeRequest<Any, EndpointTransformationOut>("GET", url.build())
    }

    /** Set or unset the transformation code associated with this endpoint. */
    suspend fun transformationPartialUpdate(
        appId: String,
        endpointId: String,
        endpointTransformationIn: EndpointTransformationIn,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/transformation")

        client.executeRequest<EndpointTransformationIn, Boolean>(
            "PATCH",
            url.build(),
            reqBody = endpointTransformationIn,
        )
    }
}
