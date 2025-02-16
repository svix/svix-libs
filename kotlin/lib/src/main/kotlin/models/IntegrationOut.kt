// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class IntegrationOut(
    val createdAt: Instant,
    val featureFlags: Set<String>? = null,
    val id: String,
    val name: String,
    val updatedAt: Instant,
)
