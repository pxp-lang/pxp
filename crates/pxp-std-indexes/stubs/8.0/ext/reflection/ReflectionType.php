<?php

abstract class ReflectionType implements \Stringable
{
    /** @implementation-alias ReflectionClass::__clone */
    final private function __clone(): void
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function allowsNull()
    {
    }
    public function __toString(): string
    {
    }
}