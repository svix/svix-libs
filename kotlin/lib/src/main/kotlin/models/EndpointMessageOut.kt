// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EndpointMessageOut(
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    val id: String,
    val nextAttempt: Instant? = null,
    val payload: JsonObject,
    val status: MessageStatus,
    val tags: Set<String>? = null,
    val timestamp: Instant,
)
