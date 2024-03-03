<?php

class ReflectionClassConstant implements \Reflector
{
    /** @implementation-alias ReflectionClass::__clone */
    final private function __clone(): void
    {
    }
    public function __construct(object|string $class, string $constant)
    {
    }
    public function __toString(): string
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getName()
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function getValue()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isPublic()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isPrivate()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isProtected()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getModifiers()
    {
    }
    /**
     * @tentative-return-type
     * @return ReflectionClass
     */
    public function getDeclaringClass()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getDocComment()
    {
    }
    /** @return ReflectionAttribute[] */
    public function getAttributes(?string $name = null, int $flags = 0): array
    {
    }
}