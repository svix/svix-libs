package com.svix.kotlin

import kotlinx.serialization.*
import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.*
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive

@OptIn(ExperimentalSerializationApi::class)
object StringAnyMapSerializer : KSerializer<Map<String, Any>> {
    override val descriptor: SerialDescriptor = JsonObject.serializer().descriptor

    override fun serialize(encoder: Encoder, value: Map<String, Any>) {
        val jsonEncoder =
            encoder as? JsonEncoder
                ?: throw SerializationException("This serializer can only be used with JSON")
        val jsonObject = value.toJsonElement()
        jsonEncoder.encodeJsonElement(jsonObject)
    }

    override fun deserialize(decoder: Decoder): Map<String, Any> {
        val jsonDecoder =
            decoder as? JsonDecoder
                ?: throw SerializationException("This serializer can only be used with JSON")
        val element = jsonDecoder.decodeJsonElement()

        if (element !is JsonObject) throw SerializationException("Expected JsonObject")

        return element.mapValues { (_, value) ->
            deserializeJsonElement(value) ?: throw SerializationException("Null values not allowed")
        }
    }

    private fun deserializeJsonElement(element: JsonElement): Any? {
        return when (element) {
            is JsonPrimitive ->
                when {
                    element.isString -> element.content
                    element.intOrNull != null -> element.int
                    element.longOrNull != null -> element.long
                    element.doubleOrNull != null -> element.double
                    element.booleanOrNull != null -> element.boolean
                    else -> element.content
                }

            is JsonArray ->
                element.map {
                    deserializeJsonElement(it)
                        ?: throw SerializationException("Null values not allowed in arrays")
                }

            is JsonObject ->
                element.mapValues { (_, value) ->
                    deserializeJsonElement(value)
                        ?: throw SerializationException("Null values not allowed in objects")
                }

            JsonNull -> null
        }
    }
}

@OptIn(ExperimentalSerializationApi::class)
object MaybeUnsetStringAnyMapSerializer : KSerializer<MaybeUnset<Map<String, Any>>> {
    override val descriptor: SerialDescriptor = JsonObject.serializer().descriptor

    override fun serialize(encoder: Encoder, value: MaybeUnset<Map<String, Any>>) {
        val jsonEncoder =
            encoder as? JsonEncoder
                ?: throw SerializationException("This serializer can only be used with JSON")

        when (value) {
            is MaybeUnset.Unset ->
                throw SerializationException("MaybeUnset.Unset should not be serialized")
            is MaybeUnset.Null -> jsonEncoder.encodeJsonElement(JsonNull)
            is MaybeUnset.Present -> StringAnyMapSerializer.serialize(encoder, value.value)
        }
    }

    override fun deserialize(decoder: Decoder): MaybeUnset<Map<String, Any>> {
        val jsonDecoder =
            decoder as? JsonDecoder
                ?: throw SerializationException("This serializer can only be used with JSON")
        val element = jsonDecoder.decodeJsonElement()

        return when (element) {
            JsonNull -> MaybeUnset.Null
            is JsonObject -> MaybeUnset.Present(StringAnyMapSerializer.deserialize(decoder))
            else -> throw SerializationException("Expected JsonObject or null")
        }
    }
}

fun Any?.toJsonElement(): JsonElement =
    when (this) {
        null -> JsonNull
        is Map<*, *> -> toJsonElement()
        is Collection<*> -> toJsonElement()
        is ByteArray -> this.toList().toJsonElement()
        is CharArray -> this.toList().toJsonElement()
        is ShortArray -> this.toList().toJsonElement()
        is IntArray -> this.toList().toJsonElement()
        is LongArray -> this.toList().toJsonElement()
        is FloatArray -> this.toList().toJsonElement()
        is DoubleArray -> this.toList().toJsonElement()
        is BooleanArray -> this.toList().toJsonElement()
        is Array<*> -> toJsonElement()
        is Boolean -> JsonPrimitive(this)
        is Number -> JsonPrimitive(this)
        is String -> JsonPrimitive(this)
        is Enum<*> -> JsonPrimitive(this.toString())
        else -> {
            throw IllegalStateException("Can't serialize unknown type: $this")
        }
    }

fun Map<*, *>.toJsonElement(): JsonElement {
    val map = mutableMapOf<String, JsonElement>()
    this.forEach { key, value ->
        key as String
        map[key] = value.toJsonElement()
    }
    return JsonObject(map)
}

fun Collection<*>.toJsonElement(): JsonElement {
    return JsonArray(this.map { it.toJsonElement() })
}

fun Array<*>.toJsonElement(): JsonElement {
    return JsonArray(this.map { it.toJsonElement() })
}
