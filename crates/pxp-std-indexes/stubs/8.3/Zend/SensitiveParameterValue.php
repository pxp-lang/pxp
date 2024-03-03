<?php

/**
 * @strict-properties
 * @not-serializable
 */
final class SensitiveParameterValue
{
    public function __construct(mixed $value)
    {
    }
    public function getValue(): mixed
    {
    }
    public function __debugInfo(): array
    {
    }
}