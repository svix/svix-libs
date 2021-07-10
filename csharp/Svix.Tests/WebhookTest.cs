using Xunit;

using System;
using System.Net;
using System.Text;
using System.Security.Cryptography;

using Svix.Exceptions;

namespace Svix.Tests {

    class TestPayload {
        private static string DEFAULT_MSG_ID = "msg_p5jXN8AQM9LWM0D4loKWxJek";
        private static string DEFAULT_PAYLOAD = "{\"test\": 2432232314}";
        private static string DEFAULT_SECRET = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

        public string id;
        public DateTimeOffset timestamp;
        public WebHeaderCollection headers;

        public string secret;
        public string payload;
        public string signature;
        public TestPayload(DateTimeOffset timestamp) {
            id = DEFAULT_MSG_ID;
            this.timestamp = timestamp;

            payload = DEFAULT_PAYLOAD;
            secret = DEFAULT_SECRET;
            
            byte[] keyBytes = Convert.FromBase64String(secret);
            string decodedSecret = Encoding.UTF8.GetString(keyBytes);
            string toSign = $"{this.id}.{this.timestamp.ToUnixTimeSeconds()}.{this.payload}";
            signature = TestPayload.Sign(decodedSecret, toSign);

            headers = new WebHeaderCollection();
            headers.Set("svix-id", id);
            headers.Set("svix-signature", $"v1,{signature}");
            headers.Set("svix-timestamp", timestamp.ToUnixTimeSeconds().ToString());
        }

        private static string Sign(string key, string payload) {
            var secretBytes = Encoding.UTF8.GetBytes(key);
            var toSignBytes = Encoding.UTF8.GetBytes(payload);

            using (var hmac = new HMACSHA256(secretBytes))
            {
                var hash = hmac.ComputeHash(toSignBytes);
                return Convert.ToBase64String(hash);
            }
        }
    }

    public class WebhookTests {
        public const int TOLERANCE_IN_SECONDS = 5 * 60;

        [Fact]
        public void TestMissingIdRasiesException() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Remove("svix-id");

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestMissingTimestampThrowsException() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Remove("svix-timestamp");

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestMissingSignatureThrowsException() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Remove("svix-signature");

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestInvalidSignatureThrowsException() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            testPayload.headers.Set("svix-signature", "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLawdd");

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestValidSignatureIsValid() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            var wh = new Webhook(testPayload.secret);

            wh.Verify(testPayload.payload, testPayload.headers);
        }

        [Fact]
        public void TestOldTimestampThrowsException() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow.AddSeconds(-1 * (TOLERANCE_IN_SECONDS + 1)));

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestNewTimestampThrowsException() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow.AddSeconds(TOLERANCE_IN_SECONDS + 1));

            var wh = new Webhook(testPayload.secret);

            Assert.Throws<WebhookVerificationException>(() => wh.Verify(testPayload.payload, testPayload.headers));
        }

        [Fact]
        public void TestMultiSigPayloadIsValid() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);
            
            string[] sigs = new string[] {
                "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
                "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
                testPayload.headers.Get("svix-signature"), // valid signature
                "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            };
            testPayload.headers.Set("svix-signature", String.Join(" ", sigs));

            var wh = new Webhook(testPayload.secret);

            wh.Verify(testPayload.payload, testPayload.headers);
        }
        
        [Fact]
        public void TestSivnatureVerificationWorksWithoutPrefix() {
            var testPayload = new TestPayload(DateTimeOffset.UtcNow);

            var wh = new Webhook(testPayload.secret);
            wh.Verify(testPayload.payload, testPayload.headers);

            wh = new Webhook("whsec_" + testPayload.secret);
            wh.Verify(testPayload.payload, testPayload.headers);
        }
    }
}
