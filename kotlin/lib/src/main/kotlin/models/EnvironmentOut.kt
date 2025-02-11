// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EnvironmentOut(
    val createdAt: Instant,
    val eventTypes: List<EventTypeOut>,
    val settings: JsonObject,
    val transformationTemplates: List<TemplateOut>,
    val version: Long? = null,
)
