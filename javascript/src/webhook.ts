import { timingSafeEqual } from "./timing_safe_equal";
import * as base64 from "@stablelib/base64";
import * as sha256 from "fast-sha256";

const WEBHOOK_TOLERANCE_IN_SECONDS = 5 * 60; // 5 minutes

class ExtendableError extends Error {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(message: any) {
    super(message);
    Object.setPrototypeOf(this, ExtendableError.prototype);
    this.name = "ExtendableError";
    this.stack = new Error(message).stack;
  }
}

export class WebhookVerificationError extends ExtendableError {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, WebhookVerificationError.prototype);
    this.name = "WebhookVerificationError";
  }
}

export interface WebhookRequiredHeaders {
  "svix-id": string;
  "svix-timestamp": string;
  "svix-signature": string;
}

export interface WebhookUnbrandedRequiredHeaders {
  "webhook-id": string;
  "webhook-timestamp": string;
  "webhook-signature": string;
}

export interface WebhookOptions {
  format?: "raw";
}

export class Webhook {
  private static prefix = "whsec_";
  private readonly key: Uint8Array;

  constructor(secret: string | Uint8Array, options?: WebhookOptions) {
    if (!secret) {
      throw new Error("Secret can't be empty.");
    }
    if (options?.format === "raw") {
      if (secret instanceof Uint8Array) {
        this.key = secret;
      } else {
        this.key = Uint8Array.from(secret, (c) => c.charCodeAt(0));
      }
    } else {
      if (typeof secret !== "string") {
        throw new Error("Expected secret to be of type string");
      }
      if (secret.startsWith(Webhook.prefix)) {
        secret = secret.substring(Webhook.prefix.length);
      }
      this.key = base64.decode(secret);
    }
  }

  public verify(
    payload: string | Buffer,
    headers_:
      | WebhookRequiredHeaders
      | WebhookUnbrandedRequiredHeaders
      | Record<string, string>
  ): unknown {
    const headers: Record<string, string> = {};
    for (const key of Object.keys(headers_)) {
      headers[key.toLowerCase()] = (headers_ as Record<string, string>)[key];
    }

    let msgId = headers["svix-id"];
    let msgSignature = headers["svix-signature"];
    let msgTimestamp = headers["svix-timestamp"];

    if (!msgSignature || !msgId || !msgTimestamp) {
      msgId = headers["webhook-id"];
      msgSignature = headers["webhook-signature"];
      msgTimestamp = headers["webhook-timestamp"];

      if (!msgSignature || !msgId || !msgTimestamp) {
        throw new WebhookVerificationError("Missing required headers");
      }
    }

    const timestamp = this.verifyTimestamp(msgTimestamp);

    const computedSignature = this.sign(msgId, timestamp, payload);
    const expectedSignature = computedSignature.split(",")[1];

    const passedSignatures = msgSignature.split(" ");

    const encoder = new globalThis.TextEncoder();
    for (const versionedSignature of passedSignatures) {
      const [version, signature] = versionedSignature.split(",");
      if (version !== "v1") {
        continue;
      }

      if (timingSafeEqual(encoder.encode(signature), encoder.encode(expectedSignature))) {
        return JSON.parse(payload.toString());
      }
    }
    throw new WebhookVerificationError("No matching signature found");
  }

  public sign(msgId: string, timestamp: Date, payload: string | Buffer): string {
    if (typeof payload === "string") {
      // Do nothing, already a string
    } else if (payload.constructor.name === "Buffer") {
      payload = payload.toString();
    } else {
      throw new Error(
        "Expected payload to be of type string or Buffer. Please refer to https://docs.svix.com/receiving/verifying-payloads/how for more information."
      );
    }

    const encoder = new TextEncoder();
    const timestampNumber = Math.floor(timestamp.getTime() / 1000);
    const toSign = encoder.encode(`${msgId}.${timestampNumber}.${payload}`);
    const expectedSignature = base64.encode(sha256.hmac(this.key, toSign));
    return `v1,${expectedSignature}`;
  }

  private verifyTimestamp(timestampHeader: string): Date {
    const now = Math.floor(Date.now() / 1000);
    const timestamp = parseInt(timestampHeader, 10);
    if (isNaN(timestamp)) {
      throw new WebhookVerificationError("Invalid Signature Headers");
    }

    if (now - timestamp > WEBHOOK_TOLERANCE_IN_SECONDS) {
      throw new WebhookVerificationError("Message timestamp too old");
    }
    if (timestamp > now + WEBHOOK_TOLERANCE_IN_SECONDS) {
      throw new WebhookVerificationError("Message timestamp too new");
    }
    return new Date(timestamp * 1000);
  }
}
