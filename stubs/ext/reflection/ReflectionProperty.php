<?php 

class ReflectionProperty implements \Reflector
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
    public function __construct(object|string $class, string $property)
    {
    }
    public function __toString(): string
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getName()
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function getValue(?object $object = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setValue(mixed $objectOrValue, mixed $value = UNKNOWN)
    {
    }
    #[\Since('8.4')]
    public function getRawValue(object $object): mixed
    {
    }
    #[\Since('8.4')]
    public function setRawValue(object $object, mixed $value): void
    {
    }
    #[\Since('8.4')]
    public function setRawValueWithoutLazyInitialization(object $object, mixed $value): void
    {
    }
    #[\Since('8.4')]
    public function skipLazyInitialization(object $object): void
    {
    }
    #[\Since('8.4')]
    public function isLazy(object $object): bool
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isInitialized(?object $object = null)
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
    #[\Since('8.4')]
    public function isPrivateSet(): bool
    {
    }
    #[\Since('8.4')]
    public function isProtectedSet(): bool
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isStatic()
    {
    }
    #[\Since('8.1')]
    public function isReadOnly(): bool
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isDefault()
    {
    }
    #[\Since('8.4')]
    public function isDynamic(): bool
    {
    }
    #[\Since('8.4')]
    public function isAbstract(): bool
    {
    }
    #[\Since('8.4')]
    public function isVirtual(): bool
    {
    }
    public function isPromoted(): bool
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
    /**
     * @tentative-return-type
     * @return void
     */
    public function setAccessible(bool $accessible)
    {
    }
    /**
     * @tentative-return-type
     * @return (ReflectionType | null)
     */
    public function getType()
    {
    }
    #[\Since('8.4')]
    public function getSettableType(): ?ReflectionType
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function hasType()
    {
    }
    public function hasDefaultValue(): bool
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function getDefaultValue()
    {
    }
    /** @return ReflectionAttribute[] */
    public function getAttributes(?string $name = null, int $flags = 0): array
    {
    }
    #[\Since('8.4')]
    public function hasHooks(): bool
    {
    }
    /** @return array<string, ReflectionMethod> */
    #[\Since('8.4')]
    public function getHooks(): array
    {
    }
    #[\Since('8.4')]
    public function hasHook(PropertyHookType $type): bool
    {
    }
    #[\Since('8.4')]
    public function getHook(PropertyHookType $type): ?ReflectionMethod
    {
    }
    #[\Since('8.4')]
    public function isFinal(): bool
    {
    }
    /**
     * @var int
     * @cvalue ZEND_ACC_STATIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_STATIC = UNKNOWN;
    /** @cvalue ZEND_ACC_STATIC */
    #[\Since('8.4')]
    public const int IS_STATIC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ACC_READONLY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_READONLY = UNKNOWN;
    /** @cvalue ZEND_ACC_READONLY */
    #[\Since('8.4')]
    public const int IS_READONLY = UNKNOWN;
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
    /** @cvalue ZEND_ACC_ABSTRACT */
    #[\Since('8.4')]
    public const int IS_ABSTRACT = UNKNOWN;
    /** @cvalue ZEND_ACC_PROTECTED_SET */
    #[\Since('8.4')]
    public const int IS_PROTECTED_SET = UNKNOWN;
    /** @cvalue ZEND_ACC_PRIVATE_SET */
    #[\Since('8.4')]
    public const int IS_PRIVATE_SET = UNKNOWN;
    /** @cvalue ZEND_ACC_VIRTUAL */
    #[\Since('8.4')]
    public const int IS_VIRTUAL = UNKNOWN;
    /** @cvalue ZEND_ACC_FINAL */
    #[\Since('8.4')]
    public const int IS_FINAL = UNKNOWN;
}