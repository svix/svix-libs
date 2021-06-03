<?php

namespace Svix;

final class WebhookTest extends \PHPUnit\Framework\TestCase {

    const SECRET = 'MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw';
    const PAYLOAD = '{"test": 2432232314}';
    const HEADER = 'svix-id=msg_p5jXN8AQM9LWM0D4loKWxJek,svix-timestamp=1614265330,svix-signature=v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=';

    public function testMissingIdThrowsException() {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");
        $header = 'svix-timestamp=1614265330,svix-signature=v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=';
        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }

    public function testMissingTimestampThrowsException() {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");
        $header = 'svix-id=msg_p5jXN8AQM9LWM0D4loKWxJek,svix-signature=v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=';
        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }

    public function testMissingSignatureThrowsException() {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("Missing required headers");
        $header = 'svix-id=msg_p5jXN8AQM9LWM0D4loKWxJek,svix-timestamp=1614265330';
        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }

    public function testInvalidSignatureThrowsException() {
        $this->expectException(\Svix\Exception\WebhookVerificationException::class);
        $this->expectExceptionMessage("No matching signature found");
        $header = 'svix-id=msg_p5jXN8AQM9LWM0D4loKWxJek,svix-timestamp=1614265330,svix-signature=v1,g0hM9SsE+OTPJTGt/dawdwdawd=';
        $wh = new Webhook(self::SECRET);
        $wh->verify(self::PAYLOAD, $header);
    }

    public function testValidSignatureIsValidAndReturnsJson() {
        $wh = new Webhook(self::SECRET);
        $json = $wh->verify(self::PAYLOAD, sefl::HEADER);
        $this->assertEquals(
            $json->{'test'},
            2432232314,
            "did not return expected json"
        );
    }
}