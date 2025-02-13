// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeImportOpenApiIn(
    val dryRun: Boolean? = null,
    val replaceAll: Boolean? = null,
    @Serializable(with = StringAnyMapSerializer::class) val spec: Map<String, Any>? = null,
    val specRaw: String? = null,
)
