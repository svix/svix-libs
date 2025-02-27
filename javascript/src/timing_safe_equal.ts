// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// https://github.com/denoland/deno_std/blob/main/crypto/timing_safe_equal.ts

/** Make an assertion, if not `true`, then throw. */
function assert(expr: unknown, msg = ""): asserts expr {
  if (!expr) {
    throw new Error(msg);
  }
}

/** Compare to array buffers or data views in a way that timing based attacks
 * cannot gain information about the platform. */
export function timingSafeEqual(
  a: ArrayBufferView | ArrayBufferLike | DataView,
  b: ArrayBufferView | ArrayBufferLike | DataView
): boolean {
  if (a.byteLength !== b.byteLength) {
    return false;
  }
  if (!(a instanceof DataView)) {
    a = ArrayBuffer.isView(a)
      ? new DataView(a.buffer, a.byteOffset, a.byteLength)
      : new DataView(a);
  }
  if (!(b instanceof DataView)) {
    b = ArrayBuffer.isView(b)
      ? new DataView(b.buffer, b.byteOffset, b.byteLength)
      : new DataView(b);
  }
  assert(a instanceof DataView);
  assert(b instanceof DataView);
  const length = a.byteLength;
  let out = 0;
  let i = -1;
  while (++i < length) {
    out |= a.getUint8(i) ^ b.getUint8(i);
  }
  return out === 0;
}
