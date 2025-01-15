<?php 

/** @generate-function-entries */
class SplPriorityQueue implements \Iterator, \Countable
{
    /**
     * @tentative-return-type
     * @return int
     */
    public function compare(mixed $priority1, mixed $priority2)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function insert(mixed $value, mixed $priority)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function setExtractFlags(int $flags)
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
    public function extract()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::count
     * @return int
     */
    public function count()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::isEmpty
     * @return bool
     */
    public function isEmpty()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::rewind
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
     * @implementation-alias SplHeap::key
     * @return int
     */
    public function key()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::next
     * @return void
     */
    public function next()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::valid
     * @return bool
     */
    public function valid()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::recoverFromCorruption
     * @return bool
     */
    public function recoverFromCorruption()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias SplHeap::isCorrupted
     * @return bool
     */
    public function isCorrupted()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getExtractFlags()
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
     * @var int
     * @cvalue SPL_PQUEUE_EXTR_BOTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EXTR_BOTH = UNKNOWN;
    /** @cvalue SPL_PQUEUE_EXTR_BOTH */
    #[\Since('8.4')]
    public const int EXTR_BOTH = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_PQUEUE_EXTR_PRIORITY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EXTR_PRIORITY = UNKNOWN;
    /** @cvalue SPL_PQUEUE_EXTR_PRIORITY */
    #[\Since('8.4')]
    public const int EXTR_PRIORITY = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_PQUEUE_EXTR_DATA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EXTR_DATA = UNKNOWN;
    /** @cvalue SPL_PQUEUE_EXTR_DATA */
    #[\Since('8.4')]
    public const int EXTR_DATA = UNKNOWN;
}