// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EventTypeFromOpenApi(
    val deprecated: Boolean,
    val description: String,
    val featureFlag: String? = null,
    val groupName: String? = null,
    val name: String,
    val schemas: JsonObject? = null,
)
