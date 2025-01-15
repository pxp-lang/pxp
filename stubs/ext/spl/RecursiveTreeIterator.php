<?php 

class RecursiveTreeIterator extends \RecursiveIteratorIterator
{
    /** @param RecursiveIterator|IteratorAggregate $iterator */
    public function __construct($iterator, int $flags = RecursiveTreeIterator::BYPASS_KEY, int $cachingIteratorFlags = CachingIterator::CATCH_GET_CHILD, int $mode = RecursiveTreeIterator::SELF_FIRST)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function key()
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
     * @return string
     */
    public function getPrefix()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setPostfix(string $postfix)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setPrefixPart(int $part, string $value)
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getEntry()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getPostfix()
    {
    }
    /**
     * @var int
     * @cvalue RTIT_BYPASS_CURRENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BYPASS_CURRENT = UNKNOWN;
    /** @cvalue RTIT_BYPASS_CURRENT */
    #[\Since('8.4')]
    public const int BYPASS_CURRENT = UNKNOWN;
    /**
     * @var int
     * @cvalue RTIT_BYPASS_KEY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BYPASS_KEY = UNKNOWN;
    /** @cvalue RTIT_BYPASS_KEY */
    #[\Since('8.4')]
    public const int BYPASS_KEY = UNKNOWN;
    /** @var int */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PREFIX_LEFT = 0;
    #[\Since('8.4')]
    public const int PREFIX_LEFT = 0;
    /** @var int */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PREFIX_MID_HAS_NEXT = 1;
    #[\Since('8.4')]
    public const int PREFIX_MID_HAS_NEXT = 1;
    /** @var int */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PREFIX_MID_LAST = 2;
    #[\Since('8.4')]
    public const int PREFIX_MID_LAST = 2;
    /** @var int */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PREFIX_END_HAS_NEXT = 3;
    #[\Since('8.4')]
    public const int PREFIX_END_HAS_NEXT = 3;
    /** @var int */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PREFIX_END_LAST = 4;
    #[\Since('8.4')]
    public const int PREFIX_END_LAST = 4;
    /** @var int */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PREFIX_RIGHT = 5;
    #[\Since('8.4')]
    public const int PREFIX_RIGHT = 5;
}