package com.svix.kotlin

import com.svix.kotlin.internal.infrastructure.ApiClient
//import com.svix.kotlin.internal.DefaultApi
//import com.svix.kotlin.internal.auth.HttpBearerAuth
// Add svix option
class Svix(token: String, svixOptions: SvixOptions = SvixOptions()) {
	companion object {
		val VERSION = "0.28.0"
	}

	val application: Application
	val authentication: Authentication
	val endpoint: Endpoint
	val eventType: EventType
	val message: Message
	val messageAttempt: MessageAttempt

	init {
		ApiClient.accessToken = token
		
		application = Application(svixOptions.debugUrl)
		authentication = Authentication(svixOptions.debugUrl)
		endpoint = Endpoint(svixOptions.debugUrl)
		eventType = EventType(svixOptions.debugUrl)
		message = Message(svixOptions.debugUrl)
		messageAttempt = MessageAttempt(svixOptions.debugUrl)
	}
}

