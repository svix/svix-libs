// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EnvironmentOut(
    val createdAt: Instant,
    val eventTypes: List<EventTypeOut>,
    @Serializable(with = StringAnyMapSerializer::class) val settings: Map<String, Any>? = null,
    val transformationTemplates: List<TemplateOut>,
    val version: Long? = null,
)
