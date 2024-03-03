<?php

namespace Random\Engine;

/**
 * @strict-properties
 * @not-serializable
 */
final class Secure implements \Random\CryptoSafeEngine
{
    /** @implementation-alias Random\Engine\Mt19937::generate */
    public function generate(): string
    {
    }
}