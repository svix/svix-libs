using System;
using System.Buffers;
using System.Buffers.Text;
using System.Net;
using System.Security.Cryptography;
using System.Text;
using Svix.Exceptions;

namespace Svix
{
    public sealed class Webhook
    {
        internal static readonly UTF8Encoding SafeUTF8Encoding = new UTF8Encoding(false, true);
        internal const string SVIX_ID_HEADER_KEY = "svix-id";
        internal const string SVIX_SIGNATURE_HEADER_KEY = "svix-signature";
        internal const string SVIX_TIMESTAMP_HEADER_KEY = "svix-timestamp";
        internal const string UNBRANDED_ID_HEADER_KEY = "webhook-id";
        internal const string UNBRANDED_SIGNATURE_HEADER_KEY = "webhook-signature";
        internal const string UNBRANDED_TIMESTAMP_HEADER_KEY = "webhook-timestamp";

        private const int SIGNATURE_LENGTH_BYTES = 32;
        private const int SIGNATURE_LENGTH_BASE64 = 48;
        private const int SIGNATURE_LENGTH_STRING = 56;
        private const int TOLERANCE_IN_SECONDS = 60 * 5;
        private const int MAX_STACKALLOC = 1024 * 256;
        private const string PREFIX = "whsec_";

        private readonly byte[] key;

        public Webhook(string key)
        {
            if (key.StartsWith(PREFIX))
            {
                key = key.Substring(PREFIX.Length);
            }

            this.key = Convert.FromBase64String(key);
        }

        public Webhook(byte[] key)
        {
            this.key = key;
        }

        public void Verify(ReadOnlySpan<char> payload, WebHeaderCollection headers)
        {
            if (payload == null)
            {
                throw new ArgumentNullException(nameof(payload));
            }
            if (headers == null)
            {
                throw new ArgumentNullException(nameof(headers));
            }

            Verify(payload, headers.Get);
        }

        public void Verify(ReadOnlySpan<char> payload, Func<string?, string?> headersProvider)
        {
            if (payload == null)
            {
                throw new ArgumentNullException(nameof(payload));
            }
            if (headersProvider == null)
            {
                throw new ArgumentNullException(nameof(headersProvider));
            }

            ReadOnlySpan<char> msgId = headersProvider(SVIX_ID_HEADER_KEY);
            ReadOnlySpan<char> msgTimestamp = headersProvider(SVIX_TIMESTAMP_HEADER_KEY);
            ReadOnlySpan<char> msgSignature = headersProvider(SVIX_SIGNATURE_HEADER_KEY);

            if (msgId.IsEmpty || msgSignature.IsEmpty || msgTimestamp.IsEmpty)
            {
                msgId = headersProvider(UNBRANDED_ID_HEADER_KEY);
                msgSignature = headersProvider(UNBRANDED_SIGNATURE_HEADER_KEY);
                msgTimestamp = headersProvider(UNBRANDED_TIMESTAMP_HEADER_KEY);
                if (msgId.IsEmpty || msgSignature.IsEmpty || msgTimestamp.IsEmpty)
                {
                    throw new WebhookVerificationException("Missing Required Headers");
                }
            }

            Webhook.VerifyTimestamp(msgTimestamp);

            Span<char> expectedSignature = stackalloc char[SIGNATURE_LENGTH_STRING];
            CalculateSignature(
                msgId,
                msgTimestamp,
                payload,
                expectedSignature,
                out var charsWritten
            );
            expectedSignature = expectedSignature.Slice(0, charsWritten);

            var signaturePtr = msgSignature;
            var spaceIndex = signaturePtr.IndexOf(' ');
            do
            {
                var versionedSignature =
                    spaceIndex < 0 ? msgSignature : signaturePtr.Slice(0, spaceIndex);

                signaturePtr = signaturePtr.Slice(spaceIndex + 1);
                spaceIndex = signaturePtr.IndexOf(' ');

                var commaIndex = versionedSignature.IndexOf(',');
                if (commaIndex < 0)
                {
                    throw new WebhookVerificationException("Invalid Signature Headers");
                }
                var version = versionedSignature.Slice(0, commaIndex);
                if (!version.Equals("v1", StringComparison.InvariantCulture))
                {
                    continue;
                }
                var passedSignature = versionedSignature.Slice(commaIndex + 1);
                if (Utils.SecureCompare(expectedSignature, passedSignature))
                {
                    return;
                }
            } while (spaceIndex >= 0);

            throw new WebhookVerificationException("No matching signature found");
        }

        private static void VerifyTimestamp(ReadOnlySpan<char> timestampHeader)
        {
            DateTimeOffset timestamp;
            var now = DateTimeOffset.UtcNow;
            try
            {
                var timestampInt = long.Parse(timestampHeader);
                timestamp = DateTimeOffset.FromUnixTimeSeconds(timestampInt);
            }
            catch
            {
                throw new WebhookVerificationException("Invalid Signature Headers");
            }

            if (timestamp < now.AddSeconds(-1 * TOLERANCE_IN_SECONDS))
            {
                throw new WebhookVerificationException("Message timestamp too old");
            }
            if (timestamp > now.AddSeconds(TOLERANCE_IN_SECONDS))
            {
                throw new WebhookVerificationException("Message timestamp too new");
            }
        }

        public string Sign(
            ReadOnlySpan<char> msgId,
            DateTimeOffset timestamp,
            ReadOnlySpan<char> payload
        )
        {
            Span<char> signature = stackalloc char[SIGNATURE_LENGTH_STRING];
            signature[0] = 'v';
            signature[1] = '1';
            signature[2] = ',';
            CalculateSignature(
                msgId,
                timestamp.ToUnixTimeSeconds().ToString(),
                payload,
                signature.Slice(3),
                out var charsWritten
            );
            return signature.Slice(0, charsWritten + 3).ToString();
        }

        private void CalculateSignature(
            ReadOnlySpan<char> msgId,
            ReadOnlySpan<char> timestamp,
            ReadOnlySpan<char> payload,
            Span<char> signature,
            out int charsWritten
        )
        {
            // Estimate buffer size and use stackalloc for smaller allocations
            int msgIdLength = SafeUTF8Encoding.GetByteCount(msgId);
            int payloadLength = SafeUTF8Encoding.GetByteCount(payload);
            int timestampLength = SafeUTF8Encoding.GetByteCount(timestamp);
            int totalLength = msgIdLength + 1 + timestampLength + 1 + payloadLength;

            Span<byte> toSignBytes =
                totalLength <= MAX_STACKALLOC
                    ? stackalloc byte[totalLength]
                    : new byte[totalLength];

            SafeUTF8Encoding.GetBytes(msgId, toSignBytes.Slice(0, msgIdLength));
            toSignBytes[msgIdLength] = (byte)'.';
            SafeUTF8Encoding.GetBytes(
                timestamp,
                toSignBytes.Slice(msgIdLength + 1, timestampLength)
            );
            toSignBytes[msgIdLength + 1 + timestampLength] = (byte)'.';
            SafeUTF8Encoding.GetBytes(
                payload,
                toSignBytes.Slice(msgIdLength + 1 + timestampLength + 1)
            );

            Span<byte> signatureBin = stackalloc byte[SIGNATURE_LENGTH_BYTES];
            CalculateSignature(toSignBytes, signatureBin);

            Span<byte> signatureB64 = stackalloc byte[SIGNATURE_LENGTH_BASE64];
            var result = Base64.EncodeToUtf8(
                signatureBin,
                signatureB64,
                out _,
                out var bytesWritten
            );
            if (result != OperationStatus.Done)
                throw new WebhookVerificationException("Failed to encode signature to base64");

            if (
                !SafeUTF8Encoding.TryGetChars(
                    signatureB64.Slice(0, bytesWritten),
                    signature,
                    out charsWritten
                )
            )
                throw new WebhookVerificationException("Failed to convert signature to utf8");
        }

        private void CalculateSignature(ReadOnlySpan<byte> input, Span<byte> output)
        {
            try
            {
                HMACSHA256.HashData(this.key, input, output);
            }
            catch (Exception)
            {
                throw new WebhookVerificationException("Output buffer too small");
            }
        }
    }
}
