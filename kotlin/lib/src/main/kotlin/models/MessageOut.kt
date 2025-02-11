// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class MessageOut(
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    val id: String,
    val payload: JsonObject,
    val tags: Set<String>? = null,
    val timestamp: Instant,
)
