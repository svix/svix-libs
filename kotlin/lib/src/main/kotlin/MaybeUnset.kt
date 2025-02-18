package com.svix.kotlin

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

@Serializable(with = MaybeUnsetSerializer::class)
sealed class MaybeUnset<out T> {
    data object Null : MaybeUnset<Nothing>()

    data object Unset : MaybeUnset<Nothing>()

    data class Present<out T>(val value: T) : MaybeUnset<T>()
}

class MaybeUnsetSerializer<T>(private val dataSerializer: KSerializer<T>) :
    KSerializer<MaybeUnset<T>> {
    override val descriptor: SerialDescriptor =
        SerialDescriptor("com.svix.kotlin.MaybeUnsetSerializer", dataSerializer.descriptor)

    @OptIn(ExperimentalSerializationApi::class)
    override fun serialize(encoder: Encoder, value: MaybeUnset<T>) {
        when (value) {
            is MaybeUnset.Unset ->
                throw SerializationException("MaybeUnset.Unset should not be serialized")

            is MaybeUnset.Null -> encoder.encodeNull()
            is MaybeUnset.Present -> encoder.encodeSerializableValue(dataSerializer, value.value)
            else -> throw SerializationException("Unreachable")
        }
    }

    override fun deserialize(decoder: Decoder): MaybeUnset<T> {
        try {
            val value = decoder.decodeSerializableValue(dataSerializer)
            return MaybeUnset.Present(value)
        } catch (e: SerializationException) {
            return MaybeUnset.Null
        }
    }
}
