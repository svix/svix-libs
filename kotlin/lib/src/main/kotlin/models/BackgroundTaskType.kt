// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.ToQueryParam
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
enum class BackgroundTaskType : ToQueryParam {
    @SerialName("endpoint.replay") ENDPOINT_REPLAY,
    @SerialName("endpoint.recover") ENDPOINT_RECOVER,
    @SerialName("application.stats") APPLICATION_STATS,
    @SerialName("message.broadcast") MESSAGE_BROADCAST,
    @SerialName("sdk.generate") SDK_GENERATE,
    @SerialName("event-type.aggregate") EVENT_TYPE_AGGREGATE,
    @SerialName("application.purge_content") APPLICATION_PURGE_CONTENT;

    override fun toQueryParam() = Json.encodeToJsonElement(this).jsonPrimitive.content
}
