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
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DONE = UNKNOWN;
    /** @cvalue BreakIterator::DONE */
    #[\Since('8.4')]
    public const int DONE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_NONE = UNKNOWN;
    /** @cvalue UBRK_WORD_NONE */
    #[\Since('8.4')]
    public const int WORD_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NONE_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_NONE_LIMIT = UNKNOWN;
    /** @cvalue UBRK_WORD_NONE_LIMIT */
    #[\Since('8.4')]
    public const int WORD_NONE_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NUMBER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_NUMBER = UNKNOWN;
    /** @cvalue UBRK_WORD_NUMBER */
    #[\Since('8.4')]
    public const int WORD_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_NUMBER_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_NUMBER_LIMIT = UNKNOWN;
    /** @cvalue UBRK_WORD_NUMBER_LIMIT */
    #[\Since('8.4')]
    public const int WORD_NUMBER_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_LETTER = UNKNOWN;
    /** @cvalue UBRK_WORD_LETTER */
    #[\Since('8.4')]
    public const int WORD_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_LETTER_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_LETTER_LIMIT = UNKNOWN;
    /** @cvalue UBRK_WORD_LETTER_LIMIT */
    #[\Since('8.4')]
    public const int WORD_LETTER_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_KANA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_KANA = UNKNOWN;
    /** @cvalue UBRK_WORD_KANA */
    #[\Since('8.4')]
    public const int WORD_KANA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_KANA_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_KANA_LIMIT = UNKNOWN;
    /** @cvalue UBRK_WORD_KANA_LIMIT */
    #[\Since('8.4')]
    public const int WORD_KANA_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_IDEO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_IDEO = UNKNOWN;
    /** @cvalue UBRK_WORD_IDEO */
    #[\Since('8.4')]
    public const int WORD_IDEO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_WORD_IDEO_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WORD_IDEO_LIMIT = UNKNOWN;
    /** @cvalue UBRK_WORD_IDEO_LIMIT */
    #[\Since('8.4')]
    public const int WORD_IDEO_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_SOFT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LINE_SOFT = UNKNOWN;
    /** @cvalue UBRK_LINE_SOFT */
    #[\Since('8.4')]
    public const int LINE_SOFT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_SOFT_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LINE_SOFT_LIMIT = UNKNOWN;
    /** @cvalue UBRK_LINE_SOFT_LIMIT */
    #[\Since('8.4')]
    public const int LINE_SOFT_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_HARD
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LINE_HARD = UNKNOWN;
    /** @cvalue UBRK_LINE_HARD */
    #[\Since('8.4')]
    public const int LINE_HARD = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_LINE_HARD_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LINE_HARD_LIMIT = UNKNOWN;
    /** @cvalue UBRK_LINE_HARD_LIMIT */
    #[\Since('8.4')]
    public const int LINE_HARD_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_TERM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SENTENCE_TERM = UNKNOWN;
    /** @cvalue UBRK_SENTENCE_TERM */
    #[\Since('8.4')]
    public const int SENTENCE_TERM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_TERM_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SENTENCE_TERM_LIMIT = UNKNOWN;
    /** @cvalue UBRK_SENTENCE_TERM_LIMIT */
    #[\Since('8.4')]
    public const int SENTENCE_TERM_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_SEP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SENTENCE_SEP = UNKNOWN;
    /** @cvalue UBRK_SENTENCE_SEP */
    #[\Since('8.4')]
    public const int SENTENCE_SEP = UNKNOWN;
    /**
     * @var int
     * @cvalue UBRK_SENTENCE_SEP_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SENTENCE_SEP_LIMIT = UNKNOWN;
    /** @cvalue UBRK_SENTENCE_SEP_LIMIT */
    #[\Since('8.4')]
    public const int SENTENCE_SEP_LIMIT = UNKNOWN;
}