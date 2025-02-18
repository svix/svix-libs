// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class MessageIn(
    val application: ApplicationIn? = null,
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    @Serializable(with = StringAnyMapSerializer::class) val payload: Map<String, Any>,
    val payloadRetentionHours: Long? = null,
    val payloadRetentionPeriod: Long? = null,
    val tags: Set<String>? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    val transformationsParams: Map<String, Any>? = null,
)
