<?php

/** @generate-function-entries */
final class Attribute
{
    public function __construct(int $flags = Attribute::TARGET_ALL)
    {
    }
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_CLASS
     */
    const TARGET_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_FUNCTION
     */
    const TARGET_FUNCTION = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_METHOD
     */
    const TARGET_METHOD = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_PROPERTY
     */
    const TARGET_PROPERTY = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_CLASS_CONST
     */
    const TARGET_CLASS_CONSTANT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_PARAMETER
     */
    const TARGET_PARAMETER = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_TARGET_ALL
     */
    const TARGET_ALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_ATTRIBUTE_IS_REPEATABLE
     */
    const IS_REPEATABLE = UNKNOWN;
}