// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import com.svix.kotlin.MaybeUnsetStringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypePatch(
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    val description: String? = null,
    val featureFlag: MaybeUnset<String> = MaybeUnset.Unset,
    val groupName: MaybeUnset<String> = MaybeUnset.Unset,
    @Serializable(with = MaybeUnsetStringAnyMapSerializer::class)
    val schemas: MaybeUnset<Map<String, Any>> = MaybeUnset.Unset,
)
