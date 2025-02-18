// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class AppUsageStatsOut(
    val id: String,
    val status: BackgroundTaskStatus,
    val task: BackgroundTaskType,
    val unresolvedAppIds: Set<String>,
)
