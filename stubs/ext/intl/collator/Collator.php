<?php 

/** @generate-function-entries */
class Collator
{
    public function __construct(string $locale)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_create
     * @return (Collator | null)
     */
    public static function create(string $locale)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_compare
     * @return (int | false)
     */
    public function compare(string $string1, string $string2)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_sort
     * @return bool
     */
    public function sort(array &$array, int $flags = Collator::SORT_REGULAR)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_sort_with_sort_keys
     * @return bool
     */
    public function sortWithSortKeys(array &$array)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_asort
     * @return bool
     */
    public function asort(array &$array, int $flags = Collator::SORT_REGULAR)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_get_attribute
     * @return (int | false)
     */
    public function getAttribute(int $attribute)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_set_attribute
     * @return bool
     */
    public function setAttribute(int $attribute, int $value)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_get_strength
     * @return int
     */
    public function getStrength()
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_set_strength
     * @return bool
     */
    public function setStrength(int $strength)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_get_locale
     * @return (string | false)
     */
    public function getLocale(int $type)
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_get_error_code
     * @return (int | false)
     */
    public function getErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_get_error_message
     * @return (string | false)
     */
    public function getErrorMessage()
    {
    }
    /**
     * @tentative-return-type
     * @alias collator_get_sort_key
     * @return (string | false)
     */
    public function getSortKey(string $string)
    {
    }
    /**
     * @var int
     * @cvalue UCOL_DEFAULT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const DEFAULT_VALUE = UNKNOWN;
    /** @cvalue UCOL_DEFAULT */
    #[\Since('8.4')]
    public const int DEFAULT_VALUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_PRIMARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const PRIMARY = UNKNOWN;
    /** @cvalue UCOL_PRIMARY */
    #[\Since('8.4')]
    public const int PRIMARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_SECONDARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const SECONDARY = UNKNOWN;
    /** @cvalue UCOL_SECONDARY */
    #[\Since('8.4')]
    public const int SECONDARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_TERTIARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const TERTIARY = UNKNOWN;
    /** @cvalue UCOL_TERTIARY */
    #[\Since('8.4')]
    public const int TERTIARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_DEFAULT_STRENGTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const DEFAULT_STRENGTH = UNKNOWN;
    /** @cvalue UCOL_DEFAULT_STRENGTH */
    #[\Since('8.4')]
    public const int DEFAULT_STRENGTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_QUATERNARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const QUATERNARY = UNKNOWN;
    /** @cvalue UCOL_QUATERNARY */
    #[\Since('8.4')]
    public const int QUATERNARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_IDENTICAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const IDENTICAL = UNKNOWN;
    /** @cvalue UCOL_IDENTICAL */
    #[\Since('8.4')]
    public const int IDENTICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_OFF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const OFF = UNKNOWN;
    /** @cvalue UCOL_OFF */
    #[\Since('8.4')]
    public const int OFF = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_ON
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const ON = UNKNOWN;
    /** @cvalue UCOL_ON */
    #[\Since('8.4')]
    public const int ON = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_SHIFTED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const SHIFTED = UNKNOWN;
    /** @cvalue UCOL_SHIFTED */
    #[\Since('8.4')]
    public const int SHIFTED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_NON_IGNORABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const NON_IGNORABLE = UNKNOWN;
    /** @cvalue UCOL_NON_IGNORABLE */
    #[\Since('8.4')]
    public const int NON_IGNORABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_LOWER_FIRST
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const LOWER_FIRST = UNKNOWN;
    /** @cvalue UCOL_LOWER_FIRST */
    #[\Since('8.4')]
    public const int LOWER_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_UPPER_FIRST
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const UPPER_FIRST = UNKNOWN;
    /** @cvalue UCOL_UPPER_FIRST */
    #[\Since('8.4')]
    public const int UPPER_FIRST = UNKNOWN;
    /* UColAttribute constants */
    /**
     * @var int
     * @cvalue UCOL_FRENCH_COLLATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const FRENCH_COLLATION = UNKNOWN;
    /* UColAttribute constants */
    /** @cvalue UCOL_FRENCH_COLLATION */
    #[\Since('8.4')]
    public const int FRENCH_COLLATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_ALTERNATE_HANDLING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const ALTERNATE_HANDLING = UNKNOWN;
    /** @cvalue UCOL_ALTERNATE_HANDLING */
    #[\Since('8.4')]
    public const int ALTERNATE_HANDLING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_CASE_FIRST
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const CASE_FIRST = UNKNOWN;
    /** @cvalue UCOL_CASE_FIRST */
    #[\Since('8.4')]
    public const int CASE_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_CASE_LEVEL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const CASE_LEVEL = UNKNOWN;
    /** @cvalue UCOL_CASE_LEVEL */
    #[\Since('8.4')]
    public const int CASE_LEVEL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_NORMALIZATION_MODE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const NORMALIZATION_MODE = UNKNOWN;
    /** @cvalue UCOL_NORMALIZATION_MODE */
    #[\Since('8.4')]
    public const int NORMALIZATION_MODE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_STRENGTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const STRENGTH = UNKNOWN;
    /** @cvalue UCOL_STRENGTH */
    #[\Since('8.4')]
    public const int STRENGTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_HIRAGANA_QUATERNARY_MODE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const HIRAGANA_QUATERNARY_MODE = UNKNOWN;
    /** @cvalue UCOL_HIRAGANA_QUATERNARY_MODE */
    #[\Since('8.4')]
    public const int HIRAGANA_QUATERNARY_MODE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_NUMERIC_COLLATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const NUMERIC_COLLATION = UNKNOWN;
    /** @cvalue UCOL_NUMERIC_COLLATION */
    #[\Since('8.4')]
    public const int NUMERIC_COLLATION = UNKNOWN;
    /* sort flags */
    /**
     * @var int
     * @cvalue COLLATOR_SORT_REGULAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const SORT_REGULAR = UNKNOWN;
    /* sort flags */
    /** @cvalue COLLATOR_SORT_REGULAR */
    #[\Since('8.4')]
    public const int SORT_REGULAR = UNKNOWN;
    /**
     * @var int
     * @cvalue COLLATOR_SORT_STRING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const SORT_STRING = UNKNOWN;
    /** @cvalue COLLATOR_SORT_STRING */
    #[\Since('8.4')]
    public const int SORT_STRING = UNKNOWN;
    /**
     * @var int
     * @cvalue COLLATOR_SORT_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    const SORT_NUMERIC = UNKNOWN;
    /** @cvalue COLLATOR_SORT_NUMERIC */
    #[\Since('8.4')]
    public const int SORT_NUMERIC = UNKNOWN;
}