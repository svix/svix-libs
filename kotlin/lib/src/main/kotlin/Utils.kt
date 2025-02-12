package com.svix.kotlin

import kotlin.reflect.full.isSubclassOf

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
        (v as ToQueryParam).toQueryParam()
    } else if (isList(v)) {
        (v as List<*>).joinToString(",")
    } else if (isSet(v)) {
        (v as Set<*>).joinToString(",")
    } else {
        v.toString()
    }
}

internal interface ToQueryParam {
    // Used to get the enums correct representation as a query param
    // does not url encode the returned string
    fun toQueryParam(): String
}
