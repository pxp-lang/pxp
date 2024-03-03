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
    public const USE_KEY = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_INVERTED
     */
    public const INVERT_MATCH = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_MATCH
     */
    public const MATCH = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_GET_MATCH
     */
    public const GET_MATCH = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_ALL_MATCHES
     */
    public const ALL_MATCHES = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_SPLIT
     */
    public const SPLIT = UNKNOWN;
    /**
     * @var int
     * @cvalue REGIT_MODE_REPLACE
     */
    public const REPLACE = UNKNOWN;
}