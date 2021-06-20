import * as utf8 from "@stablelib/utf8";
import * as base64 from "@stablelib/base64";
import * as sha256 from "fast-sha256";

import { Webhook, WebhookVerificationError } from "./index";

const defaultMsgID = "msg_p5jXN8AQM9LWM0D4loKWxJek";
const defaultPayload = `{"test": 2432232314}`;
const defaultSecret = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

const tolerance_in_ms = 5 * 60 * 1000;

class TestPayload {
  public id: string;
  public timestamp: number;
  public header: Record<string, string>;
  public secret: string;
  public payload: string;
  public signature: string;

  public constructor(timestamp = Date.now()) {
    this.id = defaultMsgID;
    this.timestamp = Math.floor(timestamp / 1000);

    this.payload = defaultPayload;
    this.secret = defaultSecret;

    const toSign = utf8.encode(`${this.id}.${this.timestamp}.${this.payload}`);
    this.signature = base64.encode(sha256.hmac(base64.decode(this.secret), toSign));

    this.header = {
      "svix-id": this.id,
      "svix-signature": "v1," + this.signature,
      "svix-timestamp": this.timestamp.toString(),
    };
  }
}

test('missing id raises error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const testPayload = new TestPayload();
    delete testPayload.header['svix-id'];

    expect(() => {
        wh.verify(testPayload.payload, testPayload.header);
    }).toThrowError(WebhookVerificationError);
});

test('missing timestamp raises error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const testPayload = new TestPayload();
    delete testPayload.header['svix-timestamp'];

    expect(() => {
        wh.verify(testPayload.payload, testPayload.header);
    }).toThrowError(WebhookVerificationError);});

test('invalid timestamp throws error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const testPayload = new TestPayload();
    testPayload.header['svix-timestamp'] = "hello";

    expect(() => {
        wh.verify(testPayload.payload, testPayload.header);
    }).toThrowError(WebhookVerificationError);});

test('missing signature raises error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const testPayload = new TestPayload();
    delete testPayload.header['svix-signature'];

    expect(() => {
        wh.verify(testPayload.payload, testPayload.header);
    }).toThrowError(WebhookVerificationError);});

test('invalid signature throws error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const testPayload = new TestPayload();
    testPayload.header['svix-signature'] = 'v1,dawfeoifkpqwoekfpqoekf';

    expect(() => {
        wh.verify(testPayload.payload, testPayload.header);
    }).toThrowError(WebhookVerificationError);
});

test('valid signature is valid and returns valid json', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const testPayload = new TestPayload();

    wh.verify(testPayload.payload, testPayload.header);
});

test("old timestamp fails", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload(Date.now() - tolerance_in_ms - 1000);

  expect(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }).toThrowError(WebhookVerificationError);
});

test("new timestamp fails", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload(Date.now() + tolerance_in_ms + 1000);

  expect(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }).toThrowError(WebhookVerificationError);
});

test("multi sig payload is valid", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  const sigs = [
    "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    testPayload.header["svix-signature"], // valid signature
    "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
  ];
  testPayload.header["svix-signature"] = sigs.join(" ");

  wh.verify(testPayload.payload, testPayload.header);
});
