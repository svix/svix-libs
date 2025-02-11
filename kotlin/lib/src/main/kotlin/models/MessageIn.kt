// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class MessageIn(
    val application: ApplicationIn? = null,
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    val payload: JsonObject,
    val payloadRetentionHours: Long? = null,
    val payloadRetentionPeriod: Long? = null,
    val tags: Set<String>? = null,
    val transformationsParams: JsonObject? = null,
)
