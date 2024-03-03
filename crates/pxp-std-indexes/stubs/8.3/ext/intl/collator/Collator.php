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
     * @return bool
     * @alias collator_set_strength
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
    const DEFAULT_VALUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_PRIMARY
     */
    const PRIMARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_SECONDARY
     */
    const SECONDARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_TERTIARY
     */
    const TERTIARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_DEFAULT_STRENGTH
     */
    const DEFAULT_STRENGTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_QUATERNARY
     */
    const QUATERNARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_IDENTICAL
     */
    const IDENTICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_OFF
     */
    const OFF = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_ON
     */
    const ON = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_SHIFTED
     */
    const SHIFTED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_NON_IGNORABLE
     */
    const NON_IGNORABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_LOWER_FIRST
     */
    const LOWER_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_UPPER_FIRST
     */
    const UPPER_FIRST = UNKNOWN;
    /* UColAttribute constants */
    /**
     * @var int
     * @cvalue UCOL_FRENCH_COLLATION
     */
    const FRENCH_COLLATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_ALTERNATE_HANDLING
     */
    const ALTERNATE_HANDLING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_CASE_FIRST
     */
    const CASE_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_CASE_LEVEL
     */
    const CASE_LEVEL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_NORMALIZATION_MODE
     */
    const NORMALIZATION_MODE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_STRENGTH
     */
    const STRENGTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_HIRAGANA_QUATERNARY_MODE
     */
    const HIRAGANA_QUATERNARY_MODE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCOL_NUMERIC_COLLATION
     */
    const NUMERIC_COLLATION = UNKNOWN;
    /* sort flags */
    /**
     * @var int
     * @cvalue COLLATOR_SORT_REGULAR
     */
    const SORT_REGULAR = UNKNOWN;
    /**
     * @var int
     * @cvalue COLLATOR_SORT_STRING
     */
    const SORT_STRING = UNKNOWN;
    /**
     * @var int
     * @cvalue COLLATOR_SORT_NUMERIC
     */
    const SORT_NUMERIC = UNKNOWN;
}