<?php 

class RegexIterator extends \FilterIterator
{
    public function __construct(Iterator $iterator, string $pattern, int $mode = RegexIterator::MATCH, int $flags = 0, int $pregFlags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function accept()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getMode()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setMode(int $mode)
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
     * @tentative-return-type
     * @return string
     */
    public function getRegex()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getPregFlags()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setPregFlags(int $pregFlags)
    {
    }
    /**
     * @var int
     * @cvalue REGIT_USE_KEY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const USE_KEY = UNKNOWN;
    /** @cvalue REGIT_USE_KEY */
    #[\Since('8.4')]
    public const int USE_KEY = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_INVERTED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const INVERT_MATCH = UNKNOWN;
    /** @cvalue REGIT_INVERTED */
    #[\Since('8.4')]
    public const int INVERT_MATCH = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_MATCH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MATCH = UNKNOWN;
    /** @cvalue REGIT_MODE_MATCH */
    #[\Since('8.4')]
    public const int MATCH = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_GET_MATCH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GET_MATCH = UNKNOWN;
    /** @cvalue REGIT_MODE_GET_MATCH */
    #[\Since('8.4')]
    public const int GET_MATCH = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_ALL_MATCHES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ALL_MATCHES = UNKNOWN;
    /** @cvalue REGIT_MODE_ALL_MATCHES */
    #[\Since('8.4')]
    public const int ALL_MATCHES = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_SPLIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SPLIT = UNKNOWN;
    /** @cvalue REGIT_MODE_SPLIT */
    #[\Since('8.4')]
    public const int SPLIT = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_REPLACE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REPLACE = UNKNOWN;
    /** @cvalue REGIT_MODE_REPLACE */
    #[\Since('8.4')]
    public const int REPLACE = UNKNOWN;
}