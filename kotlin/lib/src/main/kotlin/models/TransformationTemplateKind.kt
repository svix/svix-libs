// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
public enum class TransformationTemplateKind {
    @SerialName("Custom") CUSTOM,
    @SerialName("CustomerIO") CUSTOMER_IO,
    @SerialName("Discord") DISCORD,
    @SerialName("Hubspot") HUBSPOT,
    @SerialName("Inngest") INNGEST,
    @SerialName("Salesforce") SALESFORCE,
    @SerialName("Segment") SEGMENT,
    @SerialName("Slack") SLACK,
    @SerialName("Teams") TEAMS,
    @SerialName("TriggerDev") TRIGGER_DEV,
    @SerialName("Windmill") WINDMILL,
    @SerialName("Zapier") ZAPIER;

    override fun toString(): String = Json.encodeToJsonElement(this).jsonPrimitive.content
}
