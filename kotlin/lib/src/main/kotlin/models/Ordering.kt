// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
public enum class Ordering {
    @SerialName("ascending") ASCENDING,
    @SerialName("descending") DESCENDING;

    override fun toString(): String = Json.encodeToJsonElement(this).jsonPrimitive.content
}
