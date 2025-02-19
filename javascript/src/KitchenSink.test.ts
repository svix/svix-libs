import { Svix } from ".";
import { HttpErrorOut } from "./HttpErrors";
import { ApiException } from "./util";

function getTestClient(): Svix | null {
  const token = process.env.SVIX_TOKEN;
  const serverUrl = process.env.SVIX_SERVER_URL;
  if (!token || !serverUrl) {
    console.warn(
      "Unable to construct test client: `SVIX_TOKEN` or `SVIX_SERVER_URL` unset."
    );
    return null;
  }
  return new Svix(token, { serverUrl });
}
const client = getTestClient();

// Auto-skip tests in this module if we don't have a test client to work with.
test = client == null ? test.skip : test;

test("endpoint crud", async () => {
  const appOut = await client!.application.create({ name: "App" });
  try {
    await client!.eventType.create({
      name: "event.started",
      description: "Something started",
    });
  } catch (e) {
    // Conflicts are expected from test run to test run, but other statuses are not.
    expect((e as ApiException<HttpErrorOut>).code).toEqual(409);
  }
  try {
    await client!.eventType.create({
      name: "event.ended",
      description: "Something ended",
    });
  } catch (e) {
    // Conflicts are expected from test run to test run, but other statuses are not.
    expect((e as ApiException<HttpErrorOut>).code).toEqual(409);
  }

  const epOut = await client!.endpoint.create(appOut!.id!, {
    url: "https://example.svix.com/",
    channels: ["ch0", "ch1"],
  });
  expect(epOut!.channels!.sort()).toEqual(["ch0", "ch1"]);
  expect(epOut!.filterTypes || []).toEqual([]);

  const epPatched = await client!.endpoint.patch(appOut!.id!, epOut!.id!, {
    filterTypes: ["event.started", "event.ended"],
  });

  expect(epPatched!.channels!.sort()).toEqual(["ch0", "ch1"]);
  expect(epPatched!.filterTypes!.sort()).toEqual(["event.ended", "event.started"]);

  // Should not throw an error while trying to deserialize the empty body.
  await client!.endpoint.delete(appOut!.id!, epOut!.id!);
});
