// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

public enum class MessageStatus {
    SUCCESS,
    PENDING,
    FAIL,
    SENDING,
}

object MessageStatusSerializer : KSerializer<MessageStatus> {
    override val descriptor: SerialDescriptor =
        PrimitiveSerialDescriptor(
            "com.svix.kotlin.models.MessageStatusSerializer",
            PrimitiveKind.LONG,
        )

    override fun serialize(encoder: Encoder, value: MessageStatus) {
        val vAsLong =
            when (value) {
                MessageStatus.SUCCESS -> 0L
                MessageStatus.PENDING -> 1L
                MessageStatus.FAIL -> 2L
                MessageStatus.SENDING -> 3L
            }
        encoder.encodeLong(vAsLong)
    }

    override fun deserialize(decoder: Decoder): MessageStatus {
        return when (val vAsLong = decoder.decodeLong()) {
            0L -> MessageStatus.SUCCESS
            1L -> MessageStatus.PENDING
            2L -> MessageStatus.FAIL
            3L -> MessageStatus.SENDING
            else -> {
                throw SerializationException("$vAsLong is not a valid value for MessageStatus")
            }
        }
    }
}
