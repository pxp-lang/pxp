<?php 

class ReflectionClassConstant implements \Reflector
{
    /** @implementation-alias ReflectionClass::__clone */
    #[\Until('8.1')]
    final private function __clone(): void
    {
    }
    /** @implementation-alias ReflectionClass::__clone */
    #[\Since('8.1')]
    private function __clone(): void
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
    #[\Since('8.1')]
    public function isFinal(): bool
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
    #[\Since('8.1')]
    public function isEnumCase(): bool
    {
    }
    #[\Since('8.4')]
    public function isDeprecated(): bool
    {
    }
    #[\Since('8.3')]
    public function hasType(): bool
    {
    }
    #[\Since('8.3')]
    public function getType(): ?ReflectionType
    {
    }
    /**
     * @var int
     * @cvalue ZEND_ACC_PUBLIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_PUBLIC = UNKNOWN;
    /** @cvalue ZEND_ACC_PUBLIC */
    #[\Since('8.4')]
    public const int IS_PUBLIC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ACC_PROTECTED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_PROTECTED = UNKNOWN;
    /** @cvalue ZEND_ACC_PROTECTED */
    #[\Since('8.4')]
    public const int IS_PROTECTED = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ACC_PRIVATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_PRIVATE = UNKNOWN;
    /** @cvalue ZEND_ACC_PRIVATE */
    #[\Since('8.4')]
    public const int IS_PRIVATE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ACC_FINAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_FINAL = UNKNOWN;
    /** @cvalue ZEND_ACC_FINAL */
    #[\Since('8.4')]
    public const int IS_FINAL = UNKNOWN;
}