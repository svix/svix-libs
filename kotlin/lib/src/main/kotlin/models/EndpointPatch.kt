// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class EndpointPatch(
    val channels: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val metadata: Map<String, String>? = null,
    val rateLimit: MaybeUnset<UShort> = MaybeUnset.Unset,
    val secret: MaybeUnset<String> = MaybeUnset.Unset,
    val uid: MaybeUnset<String> = MaybeUnset.Unset,
    val url: String? = null,
    val version: UShort? = null,
)
