<?php

/**
 * @strict-properties
 */
#[\Attribute(Attribute::TARGET_PARAMETER)]
final class SensitiveParameter
{
    public function __construct()
    {
    }
}