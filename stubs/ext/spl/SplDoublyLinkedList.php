<?php 

/** @generate-function-entries */
class SplDoublyLinkedList implements \Iterator, \Countable, \ArrayAccess, \Serializable
{
    /**
     * @tentative-return-type
     * @return void
     */
    public function add(int $index, mixed $value)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function pop()
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function shift()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function push(mixed $value)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function unshift(mixed $value)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function top()
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function bottom()
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public function __debugInfo()
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
     * @tentative-return-type
     * @return bool
     */
    public function isEmpty()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function setIteratorMode(int $mode)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getIteratorMode()
    {
    }
    /**
     * @param int $index
     * @tentative-return-type
     * @return bool
     */
    public function offsetExists($index)
    {
    }
    /**
     * @param int $index
     * @tentative-return-type
     * @return mixed
     */
    public function offsetGet($index)
    {
    }
    /**
     * @param (int | null) $index
     * @tentative-return-type
     * @return void
     */
    public function offsetSet($index, mixed $value)
    {
    }
    /**
     * @param int $index
     * @tentative-return-type
     * @return void
     */
    public function offsetUnset($index)
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
     * @return mixed
     */
    public function current()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function key()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function prev()
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
    public function valid()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function unserialize(string $data)
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function serialize()
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public function __serialize()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function __unserialize(array $data)
    {
    }
    /**
     * @var int
     * @cvalue SPL_DLLIST_IT_LIFO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IT_MODE_LIFO = UNKNOWN;
    /** @cvalue SPL_DLLIST_IT_LIFO */
    #[\Since('8.4')]
    public const int IT_MODE_LIFO = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_DLLIST_IT_FIFO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IT_MODE_FIFO = UNKNOWN;
    /** @cvalue SPL_DLLIST_IT_FIFO */
    #[\Since('8.4')]
    public const int IT_MODE_FIFO = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_DLLIST_IT_DELETE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IT_MODE_DELETE = UNKNOWN;
    /** @cvalue SPL_DLLIST_IT_DELETE */
    #[\Since('8.4')]
    public const int IT_MODE_DELETE = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_DLLIST_IT_KEEP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IT_MODE_KEEP = UNKNOWN;
    /** @cvalue SPL_DLLIST_IT_KEEP */
    #[\Since('8.4')]
    public const int IT_MODE_KEEP = UNKNOWN;
}