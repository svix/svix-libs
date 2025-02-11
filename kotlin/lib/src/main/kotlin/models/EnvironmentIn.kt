// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EnvironmentIn(
    val eventTypes: List<EventTypeIn>? = null,
    val settings: JsonObject? = null,
    val transformationTemplates: List<TemplateIn>? = null,
)
