package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import kotlinx.serialization.json.Json
import okhttp3.*
import okhttp3.RequestBody.Companion.toRequestBody

open class SvixHttpClient
constructor(private val baseUrl: HttpUrl, val defaultHeaders: Map<String, String>) {
    val client: OkHttpClient = OkHttpClient()

    fun newUrlBuilder(): HttpUrl.Builder {
        return HttpUrl.Builder().scheme(baseUrl.scheme).host(baseUrl.host).port(baseUrl.port)
    }

    internal inline fun <reified Req, reified Res> executeRequest(
        method: String,
        url: HttpUrl,
        headers: Headers? = null,
        reqBody: Req? = null,
    ): Res {
        var reqBuilder = Request.Builder().url(url)

        if (reqBody != null) {
            val jsonBody = Json.encodeToString(reqBody)
            reqBuilder = reqBuilder.method(method, jsonBody.toRequestBody())
        }

        for ((k, v) in defaultHeaders) {
            reqBuilder = reqBuilder.addHeader(k, v)
        }
        if (headers != null) {
            for ((k, v) in headers) {
                reqBuilder = reqBuilder.addHeader(k, v)
            }
        }

        val request = reqBuilder.build()
        val debug: String = System.getenv("DEBUG") ?: "no"
        if (debug == "yes") {
            dbgRequest(request)
        }
        val res = client.newCall(request).execute()

        // if body is null panic
        if (res.body == null) {
            throw ApiException("Body is null", res.code)
        }
        val bodyString = res.body!!.string()
        if (debug == "yes") {
            dbgResponse(res, bodyString)
        }
        if (res.code == 204) {
            return Json.decodeFromString<Res>("true")
        }
        if (res.code in 200..299) {
            return Json.decodeFromString<Res>(bodyString)
        }
        throw ApiException("None 200 status code", res.code, bodyString)
    }
}

fun dbgRequest(request: Request) {
    println("____ start req ____")
    println("Url: ${request.url}")
    println("Method ${request.method} path: ${request.url.encodedPath}")
    for ((k, v) in request.headers) {
        println("$k: $v}")
    }
    println()
    if (request.body != null) {
        println(request.body.toString())
    }
    println("_____ end req _____")
}

fun dbgResponse(response: Response, bodyString: String?) {
    println("____ start res ____")
    println("Status: ${response.code}")
    for ((k, v) in response.headers) {
        println("$k: $v")
    }
    println()

    if (response.body != null) {
        println(bodyString)
    }
    println("_____ end res _____")
}
