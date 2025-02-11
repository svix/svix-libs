// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ListResponseMessageOut
import com.svix.kotlin.models.MessageIn
import com.svix.kotlin.models.MessageOut
import kotlinx.datetime.Instant
import okhttp3.Headers
import okhttp3.HttpUrl

data class MessageListOptions(
    val limit: ULong? = null,
    val iterator: String? = null,
    val channel: String? = null,
    val before: Instant? = null,
    val after: Instant? = null,
    val withContent: Boolean? = null,
    val tag: String? = null,
    val eventTypes: Set<String>? = null,
)

data class MessageCreateOptions(
    val withContent: Boolean? = null,
    val idempotencyKey: String? = null,
)

data class MessageGetOptions(val withContent: Boolean? = null)

class Message(baseUrl: HttpUrl, defaultHeaders: Map<String, String>) :
    SvixHttpClient(baseUrl, defaultHeaders) {

    /**
     * List all of the application's messages.
     *
     * The `before` and `after` parameters let you filter all items created before or after a
     * certain date. These can be used alongside an iterator to paginate over results within a
     * certain window.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data relative
     * to now or, if an iterator is provided, 90 days before/after the time indicated by the
     * iterator ID. If you require data beyond those time ranges, you will need to explicitly set
     * the `before` or `after` parameter as appropriate.
     */
    suspend fun list(
        appId: String,
        options: MessageListOptions = MessageListOptions(),
    ): ListResponseMessageOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/msg")
        options.limit?.let { url = url.addQueryParameter("limit", it.toString()) }
        options.iterator?.let { url = url.addQueryParameter("iterator", it) }
        options.channel?.let { url = url.addQueryParameter("channel", it) }
        options.before?.let { url = url.addQueryParameter("before", it.toString()) }
        options.after?.let { url = url.addQueryParameter("after", it.toString()) }
        options.withContent?.let { url = url.addQueryParameter("with_content", it.toString()) }
        options.tag?.let { url = url.addQueryParameter("tag", it) }
        options.eventTypes?.let { url = url.addQueryParameter("event_types", it.toString()) }
        return this.executeRequest<Any, ListResponseMessageOut>("GET", url.build())
    }

    /**
     * Creates a new message and dispatches it to all of the application's endpoints.
     *
     * The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day,
     * after that no verification will be made. If a message with the same `eventId` already exists
     * for the application, a 409 conflict error will be returned.
     *
     * The `eventType` indicates the type and schema of the event. All messages of a certain
     * `eventType` are expected to have the same schema. Endpoints can choose to only listen to
     * specific event types. Messages can also have `channels`, which similar to event types let
     * endpoints filter by them. Unlike event types, messages can have multiple channels, and
     * channels don't imply a specific message content or schema.
     *
     * The `payload` property is the webhook's body (the actual webhook message). Svix supports
     * payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads
     * small, probably no larger than 40kb.
     */
    suspend fun create(
        appId: String,
        messageIn: MessageIn,
        options: MessageCreateOptions = MessageCreateOptions(),
    ): MessageOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/msg")
        options.withContent?.let { url = url.addQueryParameter("with_content", it.toString()) }
        var headers = Headers.Builder()
        options.idempotencyKey?.let { headers = headers.add("idempotency-key", it) }

        return this.executeRequest<MessageIn, MessageOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = messageIn,
        )
    }

    /** Get a message by its ID or eventID. */
    suspend fun get(
        appId: String,
        msgId: String,
        options: MessageGetOptions = MessageGetOptions(),
    ): MessageOut {
        var url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/msg/$msgId")
        options.withContent?.let { url = url.addQueryParameter("with_content", it.toString()) }
        return this.executeRequest<Any, MessageOut>("GET", url.build())
    }

    /**
     * Delete the given message's payload.
     *
     * Useful in cases when a message was accidentally sent with sensitive content. The message
     * can't be replayed or resent once its payload has been deleted or expired.
     */
    suspend fun expungeContent(appId: String, msgId: String) {
        val url = this.newUrlBuilder().encodedPath("/api/v1/app/$appId/msg/$msgId/content")
        this.executeRequest<Any, Boolean>("DELETE", url.build())
    }
}
