// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class AppUsageStatsIn(val appIds: Set<String>? = null, val since: Instant, val until: Instant)
