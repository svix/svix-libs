// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EventTypeImportOpenApiIn(
    val dryRun: Boolean? = null,
    val replaceAll: Boolean? = null,
    val spec: JsonObject? = null,
    val specRaw: String? = null,
)
