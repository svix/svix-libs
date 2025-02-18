// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EndpointMessageOut(
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    val id: String,
    val nextAttempt: Instant? = null,
    @Serializable(with = StringAnyMapSerializer::class) val payload: Map<String, Any>,
    val status: MessageStatus,
    val tags: Set<String>? = null,
    val timestamp: Instant,
)
