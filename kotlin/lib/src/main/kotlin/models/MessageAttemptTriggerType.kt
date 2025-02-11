// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

public enum class MessageAttemptTriggerType {
    SCHEDULED,
    MANUAL,
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
