<?php 

class CachingIterator extends \IteratorIterator implements \ArrayAccess, \Countable
{
    public function __construct(Iterator $iterator, int $flags = CachingIterator::CALL_TOSTRING)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function rewind()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function valid()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function next()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function hasNext()
    {
    }
    public function __toString(): string
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getFlags()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setFlags(int $flags)
    {
    }
    /**
     * @param string $key
     * @tentative-return-type
     * @return mixed
     */
    public function offsetGet($key)
    {
    }
    /**
     * @param string $key
     * @tentative-return-type
     * @return void
     */
    public function offsetSet($key, mixed $value)
    {
    }
    /**
     * @param string $key
     * @tentative-return-type
     * @return void
     */
    public function offsetUnset($key)
    {
    }
    /**
     * @param string $key
     * @tentative-return-type
     * @return bool
     */
    public function offsetExists($key)
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public function getCache()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function count()
    {
    }
    /**
     * @var int
     * @cvalue CIT_CALL_TOSTRING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CALL_TOSTRING = UNKNOWN;
    /** @cvalue CIT_CALL_TOSTRING */
    #[\Since('8.4')]
    public const int CALL_TOSTRING = UNKNOWN;
    /**
     * @var int
     * @cvalue CIT_CATCH_GET_CHILD
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CATCH_GET_CHILD = UNKNOWN;
    /** @cvalue CIT_CATCH_GET_CHILD */
    #[\Since('8.4')]
    public const int CATCH_GET_CHILD = UNKNOWN;
    /**
     * @var int
     * @cvalue CIT_TOSTRING_USE_KEY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TOSTRING_USE_KEY = UNKNOWN;
    /** @cvalue CIT_TOSTRING_USE_KEY */
    #[\Since('8.4')]
    public const int TOSTRING_USE_KEY = UNKNOWN;
    /**
     * @var int
     * @cvalue CIT_TOSTRING_USE_CURRENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TOSTRING_USE_CURRENT = UNKNOWN;
    /** @cvalue CIT_TOSTRING_USE_CURRENT */
    #[\Since('8.4')]
    public const int TOSTRING_USE_CURRENT = UNKNOWN;
    /**
     * @var int
     * @cvalue CIT_TOSTRING_USE_INNER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TOSTRING_USE_INNER = UNKNOWN;
    /** @cvalue CIT_TOSTRING_USE_INNER */
    #[\Since('8.4')]
    public const int TOSTRING_USE_INNER = UNKNOWN;
    /**
     * @var int
     * @cvalue CIT_FULL_CACHE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FULL_CACHE = UNKNOWN;
    /** @cvalue CIT_FULL_CACHE */
    #[\Since('8.4')]
    public const int FULL_CACHE = UNKNOWN;
}