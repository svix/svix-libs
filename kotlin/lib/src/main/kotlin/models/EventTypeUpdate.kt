// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EventTypeUpdate(
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    val description: String,
    val featureFlag: String? = null,
    val groupName: String? = null,
    val schemas: JsonObject? = null,
)
