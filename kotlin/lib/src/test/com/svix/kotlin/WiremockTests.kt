package com.svix.kotlin

import com.github.tomakehurst.wiremock.WireMockServer
import com.github.tomakehurst.wiremock.client.WireMock
import com.github.tomakehurst.wiremock.client.WireMock.*
import com.github.tomakehurst.wiremock.core.WireMockConfiguration.options
import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.models.AppUsageStatsIn
import com.svix.kotlin.models.EndpointPatch
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.EventTypePatch
import com.svix.kotlin.models.MessageIn
import com.svix.kotlin.models.Ordering
import kotlin.test.assertEquals
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.Instant
import org.junit.jupiter.api.*

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class WiremockTests {

    private val wireMockServer =
        WireMockServer(options().port(0).withRootDirectory("lib/src/test/resources"))

    @BeforeAll
    fun beforeAll() {
        wireMockServer.start()
    }

    @AfterAll
    fun afterAll() {
        wireMockServer.stop()
    }

    @BeforeEach
    fun beforeEach() {
        wireMockServer.resetAll()
    }

    private fun testClient(): Svix {
        return Svix("testtk_asd12.eu", SvixOptions(this.wireMockServer.baseUrl()))
    }

    @Test
    fun testEnumQueryParamEncodedCorrectly() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app?.*"))
                .willReturn(WireMock.ok().withBodyFile("ListResponseApplicationOut.json"))
        )
        runBlocking {
            svx.application.list(
                ApplicationListOptions(
                    limit = 5UL,
                    order = Ordering.ASCENDING,
                    iterator = "cool-string-!@#$%^",
                )
            )
        }
        wireMockServer.verify(
            1,
            getRequestedFor(
                urlEqualTo(
                    "/api/v1/app?limit=5&iterator=cool-string-%21%40%23%24%25%5E&order=ascending"
                )
            ),
        )
    }

    @Test
    fun testSetQueryParamEncodedCorrectly() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app/app_asd123/msg.*"))
                .willReturn(WireMock.ok().withBodyFile("ListResponseMessageOut.json"))
        )
        runBlocking {
            svx.message.list(
                "app_asd123",
                MessageListOptions(eventTypes = setOf("key3", "key4", "key1")),
            )
        }
        wireMockServer.verify(
            1,
            getRequestedFor(urlEqualTo("/api/v1/app/app_asd123/msg?event_types=key1%2Ckey3%2Ckey4")),
        )
    }

    @Test
    fun testInstantAndBoolQueryParamEncodedCorrectly() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app/app_asd123/msg.*"))
                .willReturn(WireMock.ok().withBodyFile("ListResponseMessageOut.json"))
        )
        runBlocking {
            svx.message.list(
                "app_asd123",
                MessageListOptions(
                    before = Instant.fromEpochSeconds(1739399072, 864755000),
                    withContent = true,
                ),
            )
        }
        wireMockServer.verify(
            1,
            getRequestedFor(
                urlEqualTo(
                    "/api/v1/app/app_asd123/msg?before=2025-02-12T22%3A24%3A32.864755Z&with_content=true"
                )
            ),
        )
    }

    @Test
    fun maybeUnsetIsCorrectlySerialized() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.patch(urlMatching("/api/v1/app/ap/endpoint/endp"))
                .willReturn(WireMock.ok().withBodyFile("EndpointOut.json"))
        )
        runBlocking {
            // MaybeUnset.Present
            svx.endpoint.patch(
                "ap",
                "endp",
                EndpointPatch(filterTypes = MaybeUnset.Present(setOf("ft1", "ft2"))),
            )
            // MaybeUnset.Null
            svx.endpoint.patch("ap", "endp", EndpointPatch(filterTypes = MaybeUnset.Null))
            // MaybeUnset.Unset
            svx.endpoint.patch(
                "ap",
                "endp",
                EndpointPatch(filterTypes = MaybeUnset.Unset, version = 42u),
            )
        }
        // MaybeUnset.Present
        wireMockServer.verify(
            1,
            patchRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                .withRequestBody(equalTo("""{"filterTypes":["ft1","ft2"]}""")),
        )
        // MaybeUnset.Null
        wireMockServer.verify(
            1,
            patchRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                .withRequestBody(equalTo("""{"filterTypes":null}""")),
        )
        // MaybeUnset.Unset
        wireMockServer.verify(
            1,
            patchRequestedFor(urlEqualTo("/api/v1/app/ap/endpoint/endp"))
                .withRequestBody(equalTo("""{"version":42}""")),
        )
    }

    @Test
    fun deeplyNestedMapStringAnyIsCorrectlySerialized() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.post(urlMatching("/api/v1/event-type"))
                .willReturn(WireMock.ok().withBodyFile("EventTypeOut.json"))
        )
        val nestedList = listOf("l1", listOf("l2", listOf("l3")))
        val sampleMap =
            mapOf(
                "str_key" to "val",
                "int_key" to 1,
                "bool_key" to false,
                "list_key" to listOf("val1", "val2", nestedList),
                "map_key" to mapOf("key" to "val", "list" to nestedList),
            )
        val nestedMap =
            mapOf(
                "l1" to mapOf("l2" to mapOf("l3" to sampleMap, "more_list" to nestedList)),
                "m2" to sampleMap,
            )

        runBlocking {
            svx.eventType.create(
                EventTypeIn(description = "", name = "", schemas = mapOf("nestedMap" to nestedMap))
            )
        }
        wireMockServer.verify(
            1,
            postRequestedFor(urlEqualTo("/api/v1/event-type"))
                .withRequestBody(
                    equalToJson(
                        wireMockServer.options.stores.filesBlobStore
                            .get("VeryNestedMap.json")
                            .get()
                            .toString(Charsets.UTF_8)
                    )
                ),
        )
    }

    @Test
    fun maybeUnsetOnMapStringAnyIsCorrectlySerialized() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.patch(urlMatching("/api/v1/event-type/event"))
                .willReturn(WireMock.ok().withBodyFile("EventTypeOut.json"))
        )
        runBlocking {
            // MaybeUnset.Present
            svx.eventType.patch(
                "event",
                EventTypePatch(
                    schemas =
                        MaybeUnset.Present(
                            mapOf(
                                "str_key" to "val",
                                "int_key" to 1,
                                "bool_key" to false,
                                "list_key" to listOf("val1", "val2"),
                                "map_key" to mapOf("key" to "val"),
                            )
                        )
                ),
            )
            // MaybeUnset.Null
            svx.eventType.patch("event", EventTypePatch(schemas = MaybeUnset.Null))
            // MaybeUnset.Unset
            svx.eventType.patch(
                "event",
                EventTypePatch(description = "42", schemas = MaybeUnset.Unset),
            )
        }
        // MaybeUnset.Present
        wireMockServer.verify(
            1,
            patchRequestedFor(urlEqualTo("/api/v1/event-type/event"))
                .withRequestBody(
                    equalTo(
                        """{"schemas":{"str_key":"val","int_key":1,"bool_key":false,"list_key":["val1","val2"],"map_key":{"key":"val"}}}"""
                    )
                ),
        )
        // MaybeUnset.Null
        wireMockServer.verify(
            1,
            patchRequestedFor(urlEqualTo("/api/v1/event-type/event"))
                .withRequestBody(equalTo("""{"schemas":null}""")),
        )
        // MaybeUnset.Unset
        wireMockServer.verify(
            1,
            patchRequestedFor(urlEqualTo("/api/v1/event-type/event"))
                .withRequestBody(equalTo("""{"description":"42"}""")),
        )
    }

    @Test
    fun optionsHeadersAreSent() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.post(urlMatching("/api/v1/app/ap/msg"))
                .willReturn(WireMock.ok().withBodyFile("MessageOut.json"))
        )
        runBlocking {
            svx.message.create(
                "ap",
                MessageIn(eventType = "event.test", payload = mapOf("key" to "val")),
                MessageCreateOptions(idempotencyKey = "key123"),
            )
        }
        wireMockServer.verify(
            1,
            postRequestedFor(urlEqualTo("/api/v1/app/ap/msg"))
                .withHeader("idempotency-key", equalTo("key123")),
        )
    }

    @Test
    fun userAgentHeaderIsSent() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app/ap/msg"))
                .willReturn(WireMock.ok().withBodyFile("ListResponseMessageOut.json"))
        )
        runBlocking { svx.message.list("ap") }
        wireMockServer.verify(
            1,
            getRequestedFor(urlEqualTo("/api/v1/app/ap/msg"))
                .withHeader("User-Agent", matching("svix-libs/.*/kotlin")),
        )
    }

    @Test
    fun defaultRetryStatusCode500() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app/ap/msg"))
                .willReturn(WireMock.status(500).withBodyFile("ListResponseMessageOut.json"))
        )
        runBlocking {
            try {
                svx.message.list("ap")
            } catch (e: ApiException) {
                assertEquals(500, e.statusCode)
            }
        }

        wireMockServer.verify(
            1,
            getRequestedFor(urlEqualTo("/api/v1/app/ap/msg"))
                // first request does not have `svix-retry-count` header
                .withHeader("svix-retry-count", absent()),
        )

        // check the svix-retry-count are set correctly
        for (retryCount in 1..3) {
            wireMockServer.verify(
                1,
                getRequestedFor(urlEqualTo("/api/v1/app/ap/msg"))
                    .withHeader("svix-retry-count", equalTo("$retryCount")),
            )
        }
    }

    @Test
    fun instantSerializedCorrectly() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app/ap/msg/msg_asd123"))
                // this file includes a string timestamp `2025-02-12T22:24:32.864755Z`
                .willReturn(WireMock.ok().withBodyFile("MessageOut.json"))
        )
        runBlocking {
            val res = svx.message.get("ap", "msg_asd123")
            assertEquals(Instant.fromEpochSeconds(1739399072, 864755000), res.timestamp)
        }

        wireMockServer.verify(1, getRequestedFor(urlEqualTo("/api/v1/app/ap/msg/msg_asd123")))
    }

    @Test
    fun instantDeserializedCorrectly() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.post(urlMatching("/api/v1/stats/usage/app"))
                .willReturn(
                    WireMock.ok()
                        .withBody(
                            """{"unresolvedAppIds":["unique-identifier"],"id":"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2","status":"running","task":"endpoint.replay"}"""
                        )
                )
        )
        runBlocking {
            svx.statistics.aggregateAppStats(
                AppUsageStatsIn(
                    since = Instant.fromEpochSeconds(1739399072, 864755000),
                    until = Instant.fromEpochSeconds(1739399072, 864755000),
                )
            )
        }

        wireMockServer.verify(
            1,
            postRequestedFor(urlEqualTo("/api/v1/stats/usage/app"))
                .withRequestBody(
                    equalTo(
                        """{"since":"2025-02-12T22:24:32.864755Z","until":"2025-02-12T22:24:32.864755Z"}"""
                    )
                ),
        )
    }

    @Test
    fun listResponseOutCorrectlyDeserialized() {
        val svx = testClient()
        wireMockServer.stubFor(
            WireMock.get(urlMatching("/api/v1/app"))
                .willReturn(
                    WireMock.ok()
                        .withBody("""{"data":[],"iterator":null,"prevIterator":null,"done":true}""")
                )
        )
        runBlocking { svx.application.list() }
    }
}
