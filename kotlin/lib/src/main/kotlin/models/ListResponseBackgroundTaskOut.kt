// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ListResponseBackgroundTaskOut(
    val data: List<BackgroundTaskOut>,
    val done: Boolean,
    val iterator: String? = null,
    val prevIterator: String? = null,
)
