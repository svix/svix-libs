// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class OperationalWebhookEndpointHeadersOut(
    val headers: Map<String, String>,
    val sensitive: Set<String>,
)
