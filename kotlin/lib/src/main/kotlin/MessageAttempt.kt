// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ListResponseEndpointMessageOut
import com.svix.kotlin.models.ListResponseMessageAttemptOut
import com.svix.kotlin.models.ListResponseMessageEndpointOut
import com.svix.kotlin.models.MessageAttemptOut
import com.svix.kotlin.models.MessageStatus
import com.svix.kotlin.models.StatusCodeClass
import kotlinx.datetime.Instant
import okhttp3.Headers
import okhttp3.HttpUrl

data class MessageAttemptListByEndpointOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val status: MessageStatus? = null,
    val statusCodeClass: StatusCodeClass? = null,
    val channel: String? = null,
    val tag: String? = null,
    val before: Instant? = null,
    val after: Instant? = null,
    val withContent: Boolean? = null,
    val withMsg: Boolean? = null,
    val eventTypes: Set<String>? = null,
)

data class MessageAttemptListByMsgOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val status: MessageStatus? = null,
    val statusCodeClass: StatusCodeClass? = null,
    val channel: String? = null,
    val tag: String? = null,
    val endpointId: String? = null,
    val before: Instant? = null,
    val after: Instant? = null,
    val withContent: Boolean? = null,
    val eventTypes: Set<String>? = null,
)

data class MessageAttemptListAttemptedMessagesOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val channel: String? = null,
    val tag: String? = null,
    val status: MessageStatus? = null,
    val before: Instant? = null,
    val after: Instant? = null,
    val withContent: Boolean? = null,
    val eventTypes: Set<String>? = null,
)

data class MessageAttemptListAttemptedDestinationsOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
)

data class MessageAttemptResendOptions(val idempotencyKey: String? = null)

class MessageAttempt(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /**
     * List attempts by endpoint id
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun listByEndpoint(
        appId: String,
        endpointId: String,
        options: MessageAttemptListByEndpointOptions = MessageAttemptListByEndpointOptions(),
    ): ListResponseMessageAttemptOut {
        var url =
            this.newUrlBuilder().encodedPath("/api/v1/app/$appId/attempt/endpoint/$endpointId")
        options.limit?.let { url = url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.status?.let { url = url.addQueryParameter("status", serializeQueryParam(it)) }
        options.statusCodeClass?.let {
            url = url.addQueryParameter("status_code_class", serializeQueryParam(it))
        }
        options.channel?.let { url = url.addQueryParameter("channel", it) }
        options.tag?.let { url = url.addQueryParameter("tag", it) }
        options.before?.let { url = url.addQueryParameter("before", serializeQueryParam(it)) }
        options.after?.let { url = url.addQueryParameter("after", serializeQueryParam(it)) }
        options.withContent?.let {
            url = url.addQueryParameter("with_content", serializeQueryParam(it))
        }
        options.withMsg?.let { url = url.addQueryParameter("with_msg", serializeQueryParam(it)) }
        options.eventTypes?.let {
            url = url.addQueryParameter("event_types", serializeQueryParam(it))
        }
        return this.executeRequest<Any, ListResponseMessageAttemptOut>("GET", url.build())
    }

    /**
     * List attempts by message ID.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun listByMsg(
        appId: String,
        msgId: String,
        options: MessageAttemptListByMsgOptions = MessageAttemptListByMsgOptions(),
    ): ListResponseMessageAttemptOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/attempt/msg/$msgId")
        options.limit?.let { url = url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.status?.let { url = url.addQueryParameter("status", serializeQueryParam(it)) }
        options.statusCodeClass?.let {
            url = url.addQueryParameter("status_code_class", serializeQueryParam(it))
        }
        options.channel?.let { url = url.addQueryParameter("channel", it) }
        options.tag?.let { url = url.addQueryParameter("tag", it) }
        options.endpointId?.let { url = url.addQueryParameter("endpoint_id", it) }
        options.before?.let { url = url.addQueryParameter("before", serializeQueryParam(it)) }
        options.after?.let { url = url.addQueryParameter("after", serializeQueryParam(it)) }
        options.withContent?.let {
            url = url.addQueryParameter("with_content", serializeQueryParam(it))
        }
        options.eventTypes?.let {
            url = url.addQueryParameter("event_types", serializeQueryParam(it))
        }
        return this.executeRequest<Any, ListResponseMessageAttemptOut>("GET", url.build())
    }

    /**
     * List messages for a particular endpoint. Additionally includes metadata about the latest
     * message attempt.
     *
     * The `before` parameter lets you filter all items created before a certain date and is ignored
     * if an iterator is passed.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun listAttemptedMessages(
        appId: String,
        endpointId: String,
        options: MessageAttemptListAttemptedMessagesOptions =
            MessageAttemptListAttemptedMessagesOptions(),
    ): ListResponseEndpointMessageOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/endpoint/$endpointId/msg")
        options.limit?.let { url = url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.channel?.let { url = url.addQueryParameter("channel", it) }
        options.tag?.let { url = url.addQueryParameter("tag", it) }
        options.status?.let { url = url.addQueryParameter("status", serializeQueryParam(it)) }
        options.before?.let { url = url.addQueryParameter("before", serializeQueryParam(it)) }
        options.after?.let { url = url.addQueryParameter("after", serializeQueryParam(it)) }
        options.withContent?.let {
            url = url.addQueryParameter("with_content", serializeQueryParam(it))
        }
        options.eventTypes?.let {
            url = url.addQueryParameter("event_types", serializeQueryParam(it))
        }
        return this.executeRequest<Any, ListResponseEndpointMessageOut>("GET", url.build())
    }

    /** `msg_id`: Use a message id or a message `eventId` */
    suspend fun get(appId: String, msgId: String, attemptId: String): MessageAttemptOut {
        val url =
            this.newUrlBuilder().encodedPath("/api/v1/app/$appId/msg/$msgId/attempt/$attemptId")
        return this.executeRequest<Any, MessageAttemptOut>("GET", url.build())
    }

    /**
     * Deletes the given attempt's response body.
     *
     * Useful when an endpoint accidentally returned sensitive content. The message can't be
     * replayed or resent once its payload has been deleted or expired.
     */
    suspend fun expungeContent(appId: String, msgId: String, attemptId: String) {
        val url =
            this.newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/msg/$msgId/attempt/$attemptId/content")
        this.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /**
     * List endpoints attempted by a given message.
     *
     * Additionally includes metadata about the latest message attempt. By default, endpoints are
     * listed in ascending order by ID.
     */
    suspend fun listAttemptedDestinations(
        appId: String,
        msgId: String,
        options: MessageAttemptListAttemptedDestinationsOptions =
            MessageAttemptListAttemptedDestinationsOptions(),
    ): ListResponseMessageEndpointOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/msg/$msgId/endpoint")
        options.limit?.let { url = url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        return this.executeRequest<Any, ListResponseMessageEndpointOut>("GET", url.build())
    }

    /** Resend a message to the specified endpoint. */
    suspend fun resend(
        appId: String,
        msgId: String,
        endpointId: String,
        options: MessageAttemptResendOptions = MessageAttemptResendOptions(),
    ) {
        val url =
            this.newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/msg/$msgId/endpoint/$endpointId/resend")
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }
        this.executeRequest<Any, Boolean>("POST", url.build(), headers = headers.build())
    }
}
