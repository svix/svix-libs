// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class EndpointPatch(
    val channels: MaybeUnset<Set<String>> = MaybeUnset.Undefined,
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: MaybeUnset<Set<String>> = MaybeUnset.Undefined,
    val metadata: Map<String, String>? = null,
    val rateLimit: MaybeUnset<UShort> = MaybeUnset.Undefined,
    val secret: MaybeUnset<String> = MaybeUnset.Undefined,
    val uid: MaybeUnset<String> = MaybeUnset.Undefined,
    val url: String? = null,
    val version: UShort? = null,
)
