// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
public enum class BackgroundTaskType {
    @SerialName("endpoint.replay") ENDPOINT_REPLAY,
    @SerialName("endpoint.recover") ENDPOINT_RECOVER,
    @SerialName("application.stats") APPLICATION_STATS,
    @SerialName("message.broadcast") MESSAGE_BROADCAST,
    @SerialName("sdk.generate") SDK_GENERATE,
    @SerialName("event-type.aggregate") EVENT_TYPE_AGGREGATE;

    override fun toString(): String = Json.encodeToJsonElement(this).jsonPrimitive.content
}
