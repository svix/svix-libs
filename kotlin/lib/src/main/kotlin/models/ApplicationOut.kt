// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ApplicationOut(
    val createdAt: Instant,
    val id: String,
    val metadata: Map<String, String>,
    val name: String,
    val rateLimit: UShort? = null,
    val uid: String? = null,
    val updatedAt: Instant,
)
