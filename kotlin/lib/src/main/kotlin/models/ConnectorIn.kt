// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ConnectorIn(
    val description: String? = null,
    val featureFlag: String? = null,
    val filterTypes: Set<String>? = null,
    val instructions: String? = null,
    val instructionsLink: String? = null,
    val kind: ConnectorKind? = null,
    val logo: String,
    val name: String,
    val transformation: String,
)
