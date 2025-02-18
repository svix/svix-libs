package com.svix.kotlin

import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class Svix(token: String, options: SvixOptions = SvixOptions()) {

    val application: Application
    val authentication: Authentication
    val endpoint: Endpoint
    val eventType: EventType
    val integration: Integration
    val message: Message
    val messageAttempt: MessageAttempt
    val statistics: Statistics
    val operationalWebhookEndpoint: OperationalWebhookEndpoint

    init {
        val tokenParts = token.split(".")
        if (options.baseUrl == null) {
            val region = tokenParts.last()
            when (region) {
                "us" -> options.baseUrl = "https://api.us.svix.com"
                "eu" -> options.baseUrl = "https://api.eu.svix.com"
                "in" -> options.baseUrl = "https://api.in.svix.com"
                else -> options.baseUrl = "https://api.svix.com"
            }
        }
        val parsedUrl = options.baseUrl?.toHttpUrlOrNull() ?: throw Exception("Invalid base url")
        val defaultHeaders =
            mapOf("User-Agent" to "svix-libs/${Version}/kotlin", "Authorization" to "Bearer $token")
        val httpClient = SvixHttpClient(parsedUrl, defaultHeaders, options.retrySchedule)
        application = Application(httpClient)
        authentication = Authentication(httpClient)
        endpoint = Endpoint(httpClient)
        eventType = EventType(httpClient)
        integration = Integration(httpClient)
        message = Message(httpClient)
        messageAttempt = MessageAttempt(httpClient)
        statistics = Statistics(httpClient)
        operationalWebhookEndpoint = OperationalWebhookEndpoint(httpClient)
    }
}

data class SvixOptions(
    var baseUrl: String? = null,
    val retrySchedule: List<Long> = listOf(50, 100, 200),
) {
    init {
        if (retrySchedule.count() > 5) {
            throw IllegalArgumentException("number of retries must not exceed 5")
        }
    }
}
