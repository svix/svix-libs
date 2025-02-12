// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject

@Serializable
data class EventTypePatch(
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    val description: String? = null,
    val featureFlag: MaybeUnset<String> = MaybeUnset.Unset,
    val groupName: MaybeUnset<String> = MaybeUnset.Unset,
    val schemas: MaybeUnset<JsonObject> = MaybeUnset.Unset,
)
