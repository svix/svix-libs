// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.ToQueryParam
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
enum class Ordering : ToQueryParam {
    @SerialName("ascending") ASCENDING,
    @SerialName("descending") DESCENDING;

    override fun toQueryParam() = Json.encodeToJsonElement(this).jsonPrimitive.content
}
