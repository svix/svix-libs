// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*

public enum class StatusCodeClass {
    CODE_NONE,
    CODE1XX,
    CODE2XX,
    CODE3XX,
    CODE4XX,
    CODE5XX,
}

object StatusCodeClassSerializer : KSerializer<StatusCodeClass> {
    override val descriptor: SerialDescriptor =
        PrimitiveSerialDescriptor(
            "com.svix.kotlin.models.StatusCodeClassSerializer",
            PrimitiveKind.LONG,
        )

    override fun serialize(encoder: Encoder, value: StatusCodeClass) {
        val vAsLong =
            when (value) {
                StatusCodeClass.CODE_NONE -> 0L
                StatusCodeClass.CODE1XX -> 100L
                StatusCodeClass.CODE2XX -> 200L
                StatusCodeClass.CODE3XX -> 300L
                StatusCodeClass.CODE4XX -> 400L
                StatusCodeClass.CODE5XX -> 500L
            }
        encoder.encodeLong(vAsLong)
    }

    override fun deserialize(decoder: Decoder): StatusCodeClass {
        return when (val vAsLong = decoder.decodeLong()) {
            0L -> StatusCodeClass.CODE_NONE
            100L -> StatusCodeClass.CODE1XX
            200L -> StatusCodeClass.CODE2XX
            300L -> StatusCodeClass.CODE3XX
            400L -> StatusCodeClass.CODE4XX
            500L -> StatusCodeClass.CODE5XX
            else -> {
                throw SerializationException("$vAsLong is not a valid value for StatusCodeClass")
            }
        }
    }
}
