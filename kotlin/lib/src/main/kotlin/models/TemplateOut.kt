// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class TemplateOut(
    val createdAt: Instant,
    val description: String,
    val featureFlag: String? = null,
    val filterTypes: Set<String>? = null,
    val id: String,
    val instructions: String,
    val instructionsLink: String? = null,
    val kind: TransformationTemplateKind,
    val logo: String,
    val name: String,
    val orgId: String,
    val transformation: String,
    val updatedAt: Instant,
)
