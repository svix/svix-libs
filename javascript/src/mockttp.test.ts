import { Svix } from ".";
import { expect, test } from "@jest/globals";
import * as mockttp from "mockttp";
import { ApiException } from "./util";
import { Ordering } from "./models/ordering";

const ApplicationOut = `{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}`;
const ListResponseMessageOut = `{"data":[{"eventId":"unique-identifier","eventType":"user.signup","payload":{"email":"test@example.com","type":"user.created","username":"test_user"},"channels":["project_123","group_2"],"id":"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2","timestamp":"2019-08-24T14:15:22Z","tags":["project_1337"]}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ListResponseApplicationOut = `{"data":[{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ListResponseMessageAttemptOut = `{"data":[{"url":"https://example.com/webhook/","response":"{}","responseStatusCode":200,"responseDurationMs":0,"status":0,"triggerType":0,"msgId":"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2","endpointId":"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2","id":"atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2","timestamp":"2025-02-16T21:38:21.977Z","msg":{"eventId":"unique-identifier","eventType":"user.signup","payload":{"email":"test@example.com","type":"user.created","username":"test_user"},"channels":["project_123","group_2"],"id":"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2","timestamp":"2025-02-16T21:38:21.977Z","tags":["project_1337"]}}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ReplayOut = `{"id":"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2","status":"running","task":"endpoint.replay"}`;
const mockServer = mockttp.getLocal();

describe("mockttp tests", () => {
  beforeEach(async () => await mockServer.start(0));
  afterEach(async () => await mockServer.stop());

  test("test Date in query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app\/app1\/attempt\/endpoint\/ep1.*/)
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.messageAttempt.listByEndpoint("app1", "ep1", {
      before: new Date(1739741901977),
    });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].url.endsWith("before=2025-02-16T21%3A38%3A21.977Z")).toBe(true);
  });

  test("test Date request body", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app/app1/endpoint/ep1/replay-missing")
      .thenReply(200, ReplayOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.endpoint.replayMissing("app1", "ep1", { since: new Date(1739741901977) });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(await requests[0].body.getText()).toBe(`{"since":"2025-02-16T21:38:21.977Z"}`);
  });

  test("test Date response body", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/ep1")
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "ep1");
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(res.data[0].timestamp?.getTime()).toBe(1739741901977);
  });

  test("test listResponseOut deserializes correctly", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/ep1")
      .thenReply(200, `{"data":[],"iterator":null,"prevIterator":null,"done":true}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "ep1");
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(res.iterator).toBeNull();
    expect(res.prevIterator).toBeNull();
  });

  test("test enum as query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app.*/)
      .thenReply(200, ListResponseApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.list({ order: Ordering.Ascending });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].url.endsWith("order=ascending")).toBe(true);
  });

  test("test list as query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app\/app1\/msg.*/)
      .thenReply(200, ListResponseMessageOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.message.list("app1", { eventTypes: ["val8", "val1", "val5"] });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(
      requests[0].url.endsWith("/api/v1/app/app1/msg?event_types=val8%2Cval1%2Cval5")
    ).toBe(true);
  });

  test("test header param sent", async () => {
    const endpointMock = await mockServer
      .forPost(/\/api\/v1\/app.*/)
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.create({ name: "test" }, { idempotencyKey: "random-key" });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].headers["idempotency-key"]).toBe("random-key");
  });

  test("test retry for status => 500", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(500, `{"code":"500","detail":"asd"}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });
    await svx.application
      .list()
      .catch((error) => expect(error).toBeInstanceOf(ApiException));

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(3);

    // same svix-req-id for each retry
    const req_id = requests[0].headers["svix-req-id"];
    expect(req_id).toBeDefined();
    expect(typeof req_id).toBe("string");
    for (let i = 0; i < requests.length; i++) {
      expect(requests[i].headers["svix-req-id"]).toBe(req_id);
      if (i == 0) {
        // first request does not set svix-retry-count
        expect(requests[i].headers["svix-retry-count"]).toBeUndefined();
      } else {
        expect(requests[i].headers["svix-retry-count"]).toBe(i.toString());
      }
    }
  });
});
