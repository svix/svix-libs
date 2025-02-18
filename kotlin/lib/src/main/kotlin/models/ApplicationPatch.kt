// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class ApplicationPatch(
    val metadata: Map<String, String>? = null,
    val name: String? = null,
    val rateLimit: MaybeUnset<UShort> = MaybeUnset.Unset,
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
)
