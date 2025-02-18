// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeImportOpenApiOutData(
    val modified: List<String>,
    @SerialName("to_modify") val toModify: List<EventTypeFromOpenApi>? = null,
)
