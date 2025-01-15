<?php 

class ReflectionFunction extends \ReflectionFunctionAbstract
{
    public function __construct(Closure|string $function)
    {
    }
    public function __toString(): string
    {
    }
    #[\Since('8.2')]
    public function isAnonymous(): bool
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isDisabled()
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function invoke(mixed ...$args)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function invokeArgs(array $args)
    {
    }
    /**
     * @tentative-return-type
     * @return Closure
     */
    public function getClosure()
    {
    }
    /**
     * @var int
     * @cvalue ZEND_ACC_DEPRECATED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_DEPRECATED = UNKNOWN;
    /** @cvalue ZEND_ACC_DEPRECATED */
    #[\Since('8.4')]
    public const int IS_DEPRECATED = UNKNOWN;
}