package com.svix.kotlin

import com.github.tomakehurst.wiremock.WireMockServer
import com.github.tomakehurst.wiremock.client.WireMock
import com.github.tomakehurst.wiremock.client.WireMock.*
import com.github.tomakehurst.wiremock.core.WireMockConfiguration.options
import com.svix.kotlin.models.EndpointPatch
import com.svix.kotlin.models.Ordering
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
}
