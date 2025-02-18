// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.ToQueryParam
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable(with = MessageAttemptTriggerTypeSerializer::class)
enum class MessageAttemptTriggerType : ToQueryParam {
    SCHEDULED,
    MANUAL;

    override fun toQueryParam() = Json.encodeToJsonElement(this).jsonPrimitive.content
}

object MessageAttemptTriggerTypeSerializer : KSerializer<MessageAttemptTriggerType> {
    override val descriptor: SerialDescriptor =
        PrimitiveSerialDescriptor(
            "com.svix.kotlin.models.MessageAttemptTriggerTypeSerializer",
            PrimitiveKind.LONG,
        )

    override fun serialize(encoder: Encoder, value: MessageAttemptTriggerType) {
        val vAsLong =
            when (value) {
                MessageAttemptTriggerType.SCHEDULED -> 0L
                MessageAttemptTriggerType.MANUAL -> 1L
            }
        encoder.encodeLong(vAsLong)
    }

    override fun deserialize(decoder: Decoder): MessageAttemptTriggerType {
        return when (val vAsLong = decoder.decodeLong()) {
            0L -> MessageAttemptTriggerType.SCHEDULED
            1L -> MessageAttemptTriggerType.MANUAL
            else -> {
                throw SerializationException(
                    "$vAsLong is not a valid value for MessageAttemptTriggerType"
                )
            }
        }
    }
}
