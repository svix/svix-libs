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

@Serializable(with = MessageStatusSerializer::class)
enum class MessageStatus : ToQueryParam {
    SUCCESS,
    PENDING,
    FAIL,
    SENDING;

    override fun toQueryParam() = Json.encodeToJsonElement(this).jsonPrimitive.content
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
