package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointPatch
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.MessageIn
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.runBlocking

class BasicTest {
    companion object {
        private val AUTH_TOKEN: String? = System.getenv("SVIX_TOKEN")
        private val SERVER_URL: String? = System.getenv("SVIX_SERVER_URL")
    }

    private fun getTestClient(): Svix? =
        if (AUTH_TOKEN == null || SERVER_URL == null) {
            // TODO: figure out how to log a warning here to help teach about tests that skip when
            // the env vars are unset.
            null
        } else {
            Svix(AUTH_TOKEN, SvixOptions(SERVER_URL))
        }

    @Test
    fun basicCrudTest() {
        val svix = getTestClient() ?: return
        runBlocking {
            val applicationOut = svix.application.create(ApplicationIn(name = "App1"))
            assertEquals("App1", applicationOut.name)
            svix.message.create(
                applicationOut.id,
                MessageIn(
                    eventType = "invoice.paid",
                    payload =
                        mapOf<String, Any>(
                            "id" to "invoice_WF7WtCLFFtd8ubcTgboSFNql",
                            "status" to "paid",
                            "attempt" to 2,
                        ),
                ),
            )
            svix.application.delete(applicationOut.id)
        }
    }

    @Test
    fun kitchenSinkTest() {
        val svix = getTestClient() ?: return

        runBlocking {
            val appOut = svix.application.create(ApplicationIn(name = "App2"))
            try {
                svix.eventType.create(
                    EventTypeIn(name = "event.started", description = "Something started")
                )
            } catch (e: ApiException) {
                assertEquals(
                    409,
                    e.statusCode,
                    "conflicts are expected but other bad statuses are not",
                )
            }
            try {
                svix.eventType.create(
                    EventTypeIn(name = "event.ended", description = "Something ended")
                )
            } catch (e: ApiException) {
                assertEquals(
                    409,
                    e.statusCode,
                    "conflicts are expected but other bad statuses are not",
                )
            }

            val epOut =
                svix.endpoint.create(
                    appOut.id,
                    EndpointIn(url = "https://example.svix.com", channels = setOf("ch0", "ch1")),
                )
            assertEquals(setOf("ch0", "ch1"), epOut.channels, "initial ep should have 2 channels")
            assertEquals(0, epOut.filterTypes?.size ?: 0, "initial ep should have 0 filter types")
            val epPatched =
                svix.endpoint.patch(
                    appOut.id,
                    epOut.id,
                    EndpointPatch(
                        filterTypes = MaybeUnset.Present(setOf("event.started", "event.ended"))
                    ),
                )
            assertEquals(
                setOf("ch0", "ch1"),
                epPatched.channels,
                "patched ep should have 2 channels",
            )
            assertEquals(
                setOf("event.started", "event.ended"),
                epPatched.filterTypes,
                "patched ep should have 2 filter types",
            )
            svix.application.delete(appOut.id)
        }
    }
}
