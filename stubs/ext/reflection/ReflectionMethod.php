<?php 

class ReflectionMethod extends \ReflectionFunctionAbstract
{
    public function __construct(object|string $objectOrMethod, ?string $method = null)
    {
    }
    #[\Since('8.3')]
    public static function createFromMethodName(string $method): static
    {
    }
    public function __toString(): string
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
     * @return bool
     */
    public function isAbstract()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isFinal()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isConstructor()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isDestructor()
    {
    }
    /**
     * @tentative-return-type
     * @return Closure
     */
    public function getClosure(?object $object = null)
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
     * @return mixed
     */
    public function invoke(?object $object, mixed ...$args)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function invokeArgs(?object $object, array $args)
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
     * @return ReflectionMethod
     */
    public function getPrototype()
    {
    }
    #[\Since('8.2')]
    public function hasPrototype(): bool
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
     * @cvalue ZEND_ACC_ABSTRACT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_ABSTRACT = UNKNOWN;
    /** @cvalue ZEND_ACC_ABSTRACT */
    #[\Since('8.4')]
    public const int IS_ABSTRACT = UNKNOWN;
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