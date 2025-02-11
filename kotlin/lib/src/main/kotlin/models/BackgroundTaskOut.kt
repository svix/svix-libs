// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class BackgroundTaskOut(
    val data: JsonObject,
    val id: String,
    val status: BackgroundTaskStatus,
    val task: BackgroundTaskType,
)
