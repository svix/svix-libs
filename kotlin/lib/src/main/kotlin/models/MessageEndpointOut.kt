// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageEndpointOut(
    val channels: Set<String>? = null,
    val createdAt: Instant,
    val description: String,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    val id: String,
    val nextAttempt: Instant? = null,
    val rateLimit: UShort? = null,
    val status: MessageStatus,
    val uid: String? = null,
    val updatedAt: Instant,
    val url: String,
    val version: Int,
)
