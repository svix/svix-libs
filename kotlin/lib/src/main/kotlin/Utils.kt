package com.svix.kotlin

import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive
import kotlin.reflect.full.isSubclassOf

private fun isEnum(v: Any?): Boolean {
    return v != null && v::class.isSubclassOf(Enum::class)
}

internal fun serializeQueryParam(v: Any): String {
    return if (isEnum(v)) {
        Json.encodeToJsonElement(v).jsonPrimitive.content
    } else {
        v.toString()
    }
}
