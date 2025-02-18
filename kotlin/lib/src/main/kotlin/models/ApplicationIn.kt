// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ApplicationIn(
    val metadata: Map<String, String>? = null,
    val name: String,
    val rateLimit: UShort? = null,
    val uid: String? = null,
)
