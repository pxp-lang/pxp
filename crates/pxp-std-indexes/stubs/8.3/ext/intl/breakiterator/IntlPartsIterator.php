<?php

class IntlPartsIterator extends \IntlIterator
{
    /**
     * @tentative-return-type
     * @return IntlBreakIterator
     */
    public function getBreakIterator()
    {
    }
    /** @tentative-return-type */
    public function getRuleStatus(): int
    {
    }
    /**
     * @var int
     * @cvalue PARTS_ITERATOR_KEY_SEQUENTIAL
     */
    public const KEY_SEQUENTIAL = UNKNOWN;
    /**
     * @var int
     * @cvalue PARTS_ITERATOR_KEY_LEFT
     */
    public const KEY_LEFT = UNKNOWN;
    /**
     * @var int
     * @cvalue PARTS_ITERATOR_KEY_RIGHT
     */
    public const KEY_RIGHT = UNKNOWN;
}