// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class AppPortalAccessIn(
    val application: ApplicationIn? = null,
    val expiry: ULong? = null,
    val featureFlags: Set<String>? = null,
    val readOnly: Boolean? = null,
)
