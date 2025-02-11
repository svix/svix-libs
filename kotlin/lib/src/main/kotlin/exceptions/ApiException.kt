package com.svix.kotlin.exceptions

class ApiException(message: String? = null, val statusCode: Int = -1, val body: String? = null) :
    RuntimeException(message) {}
