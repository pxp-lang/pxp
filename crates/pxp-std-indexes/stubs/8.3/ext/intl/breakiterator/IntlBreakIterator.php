<?php

/** @generate-function-entries */
class IntlBreakIterator implements \IteratorAggregate
{
    /**
     * @tentative-return-type
     * @return (IntlBreakIterator | null)
     */
    public static function createCharacterInstance(?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @return IntlCodePointBreakIterator
     */
    public static function createCodePointInstance()
    {
    }
    /**
     * @tentative-return-type
     * @return (IntlBreakIterator | null)
     */
    public static function createLineInstance(?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (IntlBreakIterator | null)
     */
    public static function createSentenceInstance(?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (IntlBreakIterator | null)
     */
    public static function createTitleInstance(?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (IntlBreakIterator | null)
     */
    public static function createWordInstance(?string $locale = null)
    {
    }
    private function __construct()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function current()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function first()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function following(int $offset)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getErrorMessage()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getLocale(int $type)
    {
    }
    /**
     * @tentative-return-type
     * @return IntlPartsIterator
     */
    public function getPartsIterator(string $type = IntlPartsIterator::KEY_SEQUENTIAL)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function getText()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isBoundary(int $offset)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function last()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function next(?int $offset = null)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function preceding(int $offset)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function previous()
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public function setText(string $text)
    {
    }
    public function getIterator(): Iterator
    {
    }
    /**
     * @var int
     * @cvalue BreakIterator::DONE
     */
    public const DONE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NONE
     */
    public const WORD_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NONE_LIMIT
     */
    public const WORD_NONE_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NUMBER
     */
    public const WORD_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NUMBER_LIMIT
     */
    public const WORD_NUMBER_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_LETTER
     */
    public const WORD_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_LETTER_LIMIT
     */
    public const WORD_LETTER_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_KANA
     */
    public const WORD_KANA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_KANA_LIMIT
     */
    public const WORD_KANA_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_IDEO
     */
    public const WORD_IDEO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_IDEO_LIMIT
     */
    public const WORD_IDEO_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_SOFT
     */
    public const LINE_SOFT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_SOFT_LIMIT
     */
    public const LINE_SOFT_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_HARD
     */
    public const LINE_HARD = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_HARD_LIMIT
     */
    public const LINE_HARD_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_TERM
     */
    public const SENTENCE_TERM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_TERM_LIMIT
     */
    public const SENTENCE_TERM_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_SEP
     */
    public const SENTENCE_SEP = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_SEP_LIMIT
     */
    public const SENTENCE_SEP_LIMIT = UNKNOWN;
}