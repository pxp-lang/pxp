<?php 

/**
 * @strict-properties
 * @not-serializable
 */
#[\Since('8.2')]
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