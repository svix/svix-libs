// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptOut(
    val endpointId: String,
    val id: String,
    val msg: MessageOut? = null,
    val msgId: String,
    val response: String,
    val responseDurationMs: Long,
    val responseStatusCode: Short,
    val status: MessageStatus,
    val timestamp: Instant,
    val triggerType: MessageAttemptTriggerType,
    val url: String,
)
