// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ExpungeAllContentsOut(
    val id: String,
    val status: BackgroundTaskStatus,
    val task: BackgroundTaskType,
)
