// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class OperationalWebhookEndpointOut(
    val createdAt: Instant,
    val description: String,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    val id: String,
    val metadata: Map<String, String>,
    val rateLimit: UShort? = null,
    val uid: String? = null,
    val updatedAt: Instant,
    val url: String,
)
