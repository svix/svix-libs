package com.svix.test;


import com.svix.Svix;
import com.svix.SvixOptions;
import com.svix.internal.ApiException;
import com.svix.models.ApplicationIn;
import com.svix.models.ApplicationOut;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.EndpointPatch;
import com.svix.models.EventTypeIn;

import org.junit.Test;

import java.net.URI;
import java.util.Arrays;
import java.util.HashSet;

import static org.junit.Assume.assumeNotNull;
import static org.junit.Assert.assertEquals;


public class KitcheSinkTest {
	static final int HTTP_CONFLICT = 409;

	/**
	 * Builds the client to use in the test cases based on env vars.
	 * If the env vars are not set, the tests in this class are skipped (via the `assume*` calls.
	 */
	public static Svix getTestClient() {
		String token = System.getenv("SVIX_TOKEN");
		assumeNotNull(token);
		String serverUrl = System.getenv("SVIX_SERVER_URL");
		assumeNotNull(serverUrl);
		SvixOptions opts = new SvixOptions();
		opts.setServerUrl(serverUrl);
		return new Svix(token, opts);
	}

	@Test
	public void testEndpointCRUD() throws Exception {
		Svix client = getTestClient();

		ApplicationIn appIn = new ApplicationIn();
		appIn.setName("test");

		ApplicationOut app = client.getApplication().create(appIn);
		assertEquals("test", app.getName());

		EventTypeIn et1 = new EventTypeIn();
		et1.setName("event.started");
		et1.setDescription("Something started");
		try {
			client.getEventType().create(et1);
		} catch (ApiException e) {
			assertEquals(e.getCode(), HTTP_CONFLICT);
		}

		EventTypeIn et2 = new EventTypeIn();
		et2.setName("event.ended");
		et2.setDescription("Something ended");
		try {
			client.getEventType().create(et2);
		} catch (ApiException e) {
			assertEquals(e.getCode(), HTTP_CONFLICT);
		}

		EndpointIn epIn = new EndpointIn();
		epIn.setUrl(new URI("https://example.svix.com/"));
		EndpointOut ep1 = client.getEndpoint().create(app.getId(), epIn);

		EndpointPatch epPatched = new EndpointPatch();
		epPatched.addFilterTypesItem("event.started");
		epPatched.addFilterTypesItem("event.ended");
		EndpointOut ep2 = client.getEndpoint().patch(app.getId(), ep1.getId(), epPatched);

		assertEquals(
				ep2.getFilterTypes(),
				new HashSet<>(Arrays.asList("event.started", "event.ended"))
		);

		assertEquals(
				ep2.getFilterTypes(),
				new HashSet<>(Arrays.asList("event.started", "event.ended"))
		);

		// Should not throw an exception if the empty response body is handled correctly.
		client.getEndpoint().delete(app.getId(), ep1.getId());
	}
}
