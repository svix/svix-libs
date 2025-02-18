// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EventTypeImportOpenApiIn
import com.svix.kotlin.models.EventTypeImportOpenApiOut
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.EventTypeOut
import com.svix.kotlin.models.EventTypePatch
import com.svix.kotlin.models.EventTypeUpdate
import com.svix.kotlin.models.ListResponseEventTypeOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers

data class EventTypeListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
    /** When `true` archived (deleted but not expunged) items are included in the response. */
    val includeArchived: Boolean? = null,
    /** When `true` the full item (including the schema) is included in the response. */
    val withContent: Boolean? = null,
)

data class EventTypeCreateOptions(val idempotencyKey: String? = null)

data class EventTypeImportOpenapiOptions(val idempotencyKey: String? = null)

data class EventTypeDeleteOptions(
    /**
     * By default event types are archived when "deleted". Passing this to `true` deletes them
     * entirely.
     */
    val expunge: Boolean? = null
)

class EventType(private val client: SvixHttpClient) {

    /** Return the list of event types. */
    suspend fun list(
        options: EventTypeListOptions = EventTypeListOptions()
    ): ListResponseEventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        options.includeArchived?.let {
            url.addQueryParameter("include_archived", serializeQueryParam(it))
        }
        options.withContent?.let { url.addQueryParameter("with_content", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseEventTypeOut>("GET", url.build())
    }

    /**
     * Create new or unarchive existing event type.
     *
     * Unarchiving an event type will allow endpoints to filter on it and messages to be sent with
     * it. Endpoints filtering on the event type before archival will continue to filter on it. This
     * operation does not preserve the description and schemas.
     */
    suspend fun create(
        eventTypeIn: EventTypeIn,
        options: EventTypeCreateOptions = EventTypeCreateOptions(),
    ): EventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<EventTypeIn, EventTypeOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = eventTypeIn,
        )
    }

    /**
     * Given an OpenAPI spec, create new or update existing event types. If an existing `archived`
     * event type is updated, it will be unarchived.
     *
     * The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
     * top-level.
     */
    suspend fun importOpenapi(
        eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
        options: EventTypeImportOpenapiOptions = EventTypeImportOpenapiOptions(),
    ): EventTypeImportOpenApiOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type/import/openapi")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<EventTypeImportOpenApiIn, EventTypeImportOpenApiOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = eventTypeImportOpenApiIn,
        )
    }

    /** Get an event type. */
    suspend fun get(eventTypeName: String): EventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type/$eventTypeName")
        return client.executeRequest<Any, EventTypeOut>("GET", url.build())
    }

    /** Update an event type. */
    suspend fun update(eventTypeName: String, eventTypeUpdate: EventTypeUpdate): EventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type/$eventTypeName")

        return client.executeRequest<EventTypeUpdate, EventTypeOut>(
            "PUT",
            url.build(),
            reqBody = eventTypeUpdate,
        )
    }

    /**
     * Archive an event type.
     *
     * Endpoints already configured to filter on an event type will continue to do so after
     * archival. However, new messages can not be sent with it and endpoints can not filter on it.
     * An event type can be unarchived with the
     * [create operation](#operation/create_event_type_api_v1_event_type__post).
     */
    suspend fun delete(
        eventTypeName: String,
        options: EventTypeDeleteOptions = EventTypeDeleteOptions(),
    ) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type/$eventTypeName")
        options.expunge?.let { url.addQueryParameter("expunge", serializeQueryParam(it)) }
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update an event type. */
    suspend fun patch(eventTypeName: String, eventTypePatch: EventTypePatch): EventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/event-type/$eventTypeName")

        return client.executeRequest<EventTypePatch, EventTypeOut>(
            "PATCH",
            url.build(),
            reqBody = eventTypePatch,
        )
    }
}
