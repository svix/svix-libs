package com.svix.kotlin

import kotlin.reflect.full.isSubclassOf
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

private fun isEnum(v: Any?): Boolean {
    return v != null && v::class.isSubclassOf(Enum::class)
}

private fun isList(v: Any?): Boolean {
    return v != null && v::class.isSubclassOf(List::class)
}

private fun isSet(v: Any?): Boolean {
    return v != null && v::class.isSubclassOf(Set::class)
}

internal fun serializeQueryParam(v: Any): String {
    return if (isEnum(v)) {
        Json.encodeToJsonElement(v).jsonPrimitive.content
    } else if (isList(v)) {
        (v as List<*>).joinToString(",")
    } else if (isSet(v)) {
        (v as Set<*>).joinToString(",")
    } else {
        v.toString()
    }
}
