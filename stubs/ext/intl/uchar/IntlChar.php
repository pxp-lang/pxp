<?php 

/** @generate-function-entries */
class IntlChar
{
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function hasBinaryProperty(int|string $codepoint, int $property)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | null)
     */
    public static function charAge(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function charDigitValue(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function charDirection(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function charFromName(string $name, int $type = IntlChar::UNICODE_CHAR_NAME)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | string | null)
     */
    public static function charMirror(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public static function charName(int|string $codepoint, int $type = IntlChar::UNICODE_CHAR_NAME)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function charType(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public static function chr(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false | null)
     */
    public static function digit(int|string $codepoint, int $base = 10)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function enumCharNames(int|string $start, int|string $end, callable $callback, int $type = IntlChar::UNICODE_CHAR_NAME)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public static function enumCharTypes(callable $callback)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | string | null)
     */
    public static function foldCase(int|string $codepoint, int $options = IntlChar::FOLD_CASE_DEFAULT)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public static function forDigit(int $digit, int $base = 10)
    {
    }
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @tentative-return-type
     * @return (int | string | null)
     */
    public static function getBidiPairedBracket(int|string $codepoint)
    {
    }
    #endif
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function getBlockCode(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function getCombiningClass(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false | null)
     */
    public static function getFC_NFKC_Closure(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public static function getIntPropertyMaxValue(int $property)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public static function getIntPropertyMinValue(int $property)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function getIntPropertyValue(int|string $codepoint, int $property)
    {
    }
    /**
     * @tentative-return-type
     * @return (float | null)
     */
    public static function getNumericValue(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public static function getPropertyEnum(string $alias)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public static function getPropertyName(int $property, int $type = IntlChar::LONG_PROPERTY_NAME)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public static function getPropertyValueEnum(int $property, string $name)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public static function getPropertyValueName(int $property, int $value, int $type = IntlChar::LONG_PROPERTY_NAME)
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public static function getUnicodeVersion()
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isalnum(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isalpha(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isbase(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isblank(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function iscntrl(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isdefined(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isdigit(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isgraph(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isIDIgnorable(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isIDPart(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isIDStart(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isISOControl(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isJavaIDPart(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isJavaIDStart(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isJavaSpaceChar(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function islower(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isMirrored(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isprint(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function ispunct(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isspace(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function istitle(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isUAlphabetic(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isULowercase(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isupper(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isUUppercase(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isUWhiteSpace(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isWhitespace(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | null)
     */
    public static function isxdigit(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | null)
     */
    public static function ord(int|string $character)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | string | null)
     */
    public static function tolower(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | string | null)
     */
    public static function totitle(int|string $codepoint)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | string | null)
     */
    public static function toupper(int|string $codepoint)
    {
    }
    /**
     * @var string
     * @cvalue U_UNICODE_VERSION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UNICODE_VERSION = UNKNOWN;
    /** @cvalue U_UNICODE_VERSION */
    #[\Since('8.4')]
    public const string UNICODE_VERSION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_MIN_VALUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CODEPOINT_MIN = UNKNOWN;
    /** @cvalue UCHAR_MIN_VALUE */
    #[\Since('8.4')]
    public const int CODEPOINT_MIN = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_MAX_VALUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CODEPOINT_MAX = UNKNOWN;
    /** @cvalue UCHAR_MAX_VALUE */
    #[\Since('8.4')]
    public const int CODEPOINT_MAX = UNKNOWN;
    /**
     * @var float
     * @cvalue U_NO_NUMERIC_VALUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NO_NUMERIC_VALUE = UNKNOWN;
    /** @cvalue U_NO_NUMERIC_VALUE */
    #[\Since('8.4')]
    public const float NO_NUMERIC_VALUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_ALPHABETIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_ALPHABETIC = UNKNOWN;
    /** @cvalue UCHAR_ALPHABETIC */
    #[\Since('8.4')]
    public const int PROPERTY_ALPHABETIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BINARY_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BINARY_START = UNKNOWN;
    /** @cvalue UCHAR_BINARY_START */
    #[\Since('8.4')]
    public const int PROPERTY_BINARY_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_ASCII_HEX_DIGIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_ASCII_HEX_DIGIT = UNKNOWN;
    /** @cvalue UCHAR_ASCII_HEX_DIGIT */
    #[\Since('8.4')]
    public const int PROPERTY_ASCII_HEX_DIGIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BIDI_CONTROL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BIDI_CONTROL = UNKNOWN;
    /** @cvalue UCHAR_BIDI_CONTROL */
    #[\Since('8.4')]
    public const int PROPERTY_BIDI_CONTROL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BIDI_MIRRORED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BIDI_MIRRORED = UNKNOWN;
    /** @cvalue UCHAR_BIDI_MIRRORED */
    #[\Since('8.4')]
    public const int PROPERTY_BIDI_MIRRORED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DASH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DASH = UNKNOWN;
    /** @cvalue UCHAR_DASH */
    #[\Since('8.4')]
    public const int PROPERTY_DASH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DEFAULT_IGNORABLE_CODE_POINT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DEFAULT_IGNORABLE_CODE_POINT = UNKNOWN;
    /** @cvalue UCHAR_DEFAULT_IGNORABLE_CODE_POINT */
    #[\Since('8.4')]
    public const int PROPERTY_DEFAULT_IGNORABLE_CODE_POINT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DEPRECATED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DEPRECATED = UNKNOWN;
    /** @cvalue UCHAR_DEPRECATED */
    #[\Since('8.4')]
    public const int PROPERTY_DEPRECATED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DIACRITIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DIACRITIC = UNKNOWN;
    /** @cvalue UCHAR_DIACRITIC */
    #[\Since('8.4')]
    public const int PROPERTY_DIACRITIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_EXTENDER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_EXTENDER = UNKNOWN;
    /** @cvalue UCHAR_EXTENDER */
    #[\Since('8.4')]
    public const int PROPERTY_EXTENDER = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_FULL_COMPOSITION_EXCLUSION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_FULL_COMPOSITION_EXCLUSION = UNKNOWN;
    /** @cvalue UCHAR_FULL_COMPOSITION_EXCLUSION */
    #[\Since('8.4')]
    public const int PROPERTY_FULL_COMPOSITION_EXCLUSION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_GRAPHEME_BASE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_GRAPHEME_BASE = UNKNOWN;
    /** @cvalue UCHAR_GRAPHEME_BASE */
    #[\Since('8.4')]
    public const int PROPERTY_GRAPHEME_BASE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_GRAPHEME_EXTEND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_GRAPHEME_EXTEND = UNKNOWN;
    /** @cvalue UCHAR_GRAPHEME_EXTEND */
    #[\Since('8.4')]
    public const int PROPERTY_GRAPHEME_EXTEND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_GRAPHEME_LINK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_GRAPHEME_LINK = UNKNOWN;
    /** @cvalue UCHAR_GRAPHEME_LINK */
    #[\Since('8.4')]
    public const int PROPERTY_GRAPHEME_LINK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_HEX_DIGIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_HEX_DIGIT = UNKNOWN;
    /** @cvalue UCHAR_HEX_DIGIT */
    #[\Since('8.4')]
    public const int PROPERTY_HEX_DIGIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_HYPHEN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_HYPHEN = UNKNOWN;
    /** @cvalue UCHAR_HYPHEN */
    #[\Since('8.4')]
    public const int PROPERTY_HYPHEN = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_ID_CONTINUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_ID_CONTINUE = UNKNOWN;
    /** @cvalue UCHAR_ID_CONTINUE */
    #[\Since('8.4')]
    public const int PROPERTY_ID_CONTINUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_ID_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_ID_START = UNKNOWN;
    /** @cvalue UCHAR_ID_START */
    #[\Since('8.4')]
    public const int PROPERTY_ID_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_IDEOGRAPHIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_IDEOGRAPHIC = UNKNOWN;
    /** @cvalue UCHAR_IDEOGRAPHIC */
    #[\Since('8.4')]
    public const int PROPERTY_IDEOGRAPHIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_IDS_BINARY_OPERATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_IDS_BINARY_OPERATOR = UNKNOWN;
    /** @cvalue UCHAR_IDS_BINARY_OPERATOR */
    #[\Since('8.4')]
    public const int PROPERTY_IDS_BINARY_OPERATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_IDS_TRINARY_OPERATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_IDS_TRINARY_OPERATOR = UNKNOWN;
    /** @cvalue UCHAR_IDS_TRINARY_OPERATOR */
    #[\Since('8.4')]
    public const int PROPERTY_IDS_TRINARY_OPERATOR = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 74
    /** @cvalue UCHAR_IDS_UNARY_OPERATOR */
    #[\Since('8.4')]
    public const int PROPERTY_IDS_UNARY_OPERATOR = UNKNOWN;
    /** @cvalue UCHAR_ID_COMPAT_MATH_START */
    #[\Since('8.4')]
    public const int PROPERTY_ID_COMPAT_MATH_START = UNKNOWN;
    /** @cvalue UCHAR_ID_COMPAT_MATH_CONTINUE */
    #[\Since('8.4')]
    public const int PROPERTY_ID_COMPAT_MATH_CONTINUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_JOIN_CONTROL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_JOIN_CONTROL = UNKNOWN;
    #endif
    /** @cvalue UCHAR_JOIN_CONTROL */
    #[\Since('8.4')]
    public const int PROPERTY_JOIN_CONTROL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_LOGICAL_ORDER_EXCEPTION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_LOGICAL_ORDER_EXCEPTION = UNKNOWN;
    /** @cvalue UCHAR_LOGICAL_ORDER_EXCEPTION */
    #[\Since('8.4')]
    public const int PROPERTY_LOGICAL_ORDER_EXCEPTION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_LOWERCASE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_LOWERCASE = UNKNOWN;
    /** @cvalue UCHAR_LOWERCASE */
    #[\Since('8.4')]
    public const int PROPERTY_LOWERCASE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_MATH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_MATH = UNKNOWN;
    /** @cvalue UCHAR_MATH */
    #[\Since('8.4')]
    public const int PROPERTY_MATH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NONCHARACTER_CODE_POINT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NONCHARACTER_CODE_POINT = UNKNOWN;
    /** @cvalue UCHAR_NONCHARACTER_CODE_POINT */
    #[\Since('8.4')]
    public const int PROPERTY_NONCHARACTER_CODE_POINT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_QUOTATION_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_QUOTATION_MARK = UNKNOWN;
    /** @cvalue UCHAR_QUOTATION_MARK */
    #[\Since('8.4')]
    public const int PROPERTY_QUOTATION_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_RADICAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_RADICAL = UNKNOWN;
    /** @cvalue UCHAR_RADICAL */
    #[\Since('8.4')]
    public const int PROPERTY_RADICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SOFT_DOTTED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SOFT_DOTTED = UNKNOWN;
    /** @cvalue UCHAR_SOFT_DOTTED */
    #[\Since('8.4')]
    public const int PROPERTY_SOFT_DOTTED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_TERMINAL_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_TERMINAL_PUNCTUATION = UNKNOWN;
    /** @cvalue UCHAR_TERMINAL_PUNCTUATION */
    #[\Since('8.4')]
    public const int PROPERTY_TERMINAL_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_UNIFIED_IDEOGRAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_UNIFIED_IDEOGRAPH = UNKNOWN;
    /** @cvalue UCHAR_UNIFIED_IDEOGRAPH */
    #[\Since('8.4')]
    public const int PROPERTY_UNIFIED_IDEOGRAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_UPPERCASE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_UPPERCASE = UNKNOWN;
    /** @cvalue UCHAR_UPPERCASE */
    #[\Since('8.4')]
    public const int PROPERTY_UPPERCASE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_WHITE_SPACE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_WHITE_SPACE = UNKNOWN;
    /** @cvalue UCHAR_WHITE_SPACE */
    #[\Since('8.4')]
    public const int PROPERTY_WHITE_SPACE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_XID_CONTINUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_XID_CONTINUE = UNKNOWN;
    /** @cvalue UCHAR_XID_CONTINUE */
    #[\Since('8.4')]
    public const int PROPERTY_XID_CONTINUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_XID_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_XID_START = UNKNOWN;
    /** @cvalue UCHAR_XID_START */
    #[\Since('8.4')]
    public const int PROPERTY_XID_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CASE_SENSITIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CASE_SENSITIVE = UNKNOWN;
    /** @cvalue UCHAR_CASE_SENSITIVE */
    #[\Since('8.4')]
    public const int PROPERTY_CASE_SENSITIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_S_TERM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_S_TERM = UNKNOWN;
    /** @cvalue UCHAR_S_TERM */
    #[\Since('8.4')]
    public const int PROPERTY_S_TERM = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_VARIATION_SELECTOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_VARIATION_SELECTOR = UNKNOWN;
    /** @cvalue UCHAR_VARIATION_SELECTOR */
    #[\Since('8.4')]
    public const int PROPERTY_VARIATION_SELECTOR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFD_INERT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFD_INERT = UNKNOWN;
    /** @cvalue UCHAR_NFD_INERT */
    #[\Since('8.4')]
    public const int PROPERTY_NFD_INERT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFKD_INERT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFKD_INERT = UNKNOWN;
    /** @cvalue UCHAR_NFKD_INERT */
    #[\Since('8.4')]
    public const int PROPERTY_NFKD_INERT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFC_INERT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFC_INERT = UNKNOWN;
    /** @cvalue UCHAR_NFC_INERT */
    #[\Since('8.4')]
    public const int PROPERTY_NFC_INERT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFKC_INERT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFKC_INERT = UNKNOWN;
    /** @cvalue UCHAR_NFKC_INERT */
    #[\Since('8.4')]
    public const int PROPERTY_NFKC_INERT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SEGMENT_STARTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SEGMENT_STARTER = UNKNOWN;
    /** @cvalue UCHAR_SEGMENT_STARTER */
    #[\Since('8.4')]
    public const int PROPERTY_SEGMENT_STARTER = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_PATTERN_SYNTAX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_PATTERN_SYNTAX = UNKNOWN;
    /** @cvalue UCHAR_PATTERN_SYNTAX */
    #[\Since('8.4')]
    public const int PROPERTY_PATTERN_SYNTAX = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_PATTERN_WHITE_SPACE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_PATTERN_WHITE_SPACE = UNKNOWN;
    /** @cvalue UCHAR_PATTERN_WHITE_SPACE */
    #[\Since('8.4')]
    public const int PROPERTY_PATTERN_WHITE_SPACE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_POSIX_ALNUM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_POSIX_ALNUM = UNKNOWN;
    /** @cvalue UCHAR_POSIX_ALNUM */
    #[\Since('8.4')]
    public const int PROPERTY_POSIX_ALNUM = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_POSIX_BLANK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_POSIX_BLANK = UNKNOWN;
    /** @cvalue UCHAR_POSIX_BLANK */
    #[\Since('8.4')]
    public const int PROPERTY_POSIX_BLANK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_POSIX_GRAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_POSIX_GRAPH = UNKNOWN;
    /** @cvalue UCHAR_POSIX_GRAPH */
    #[\Since('8.4')]
    public const int PROPERTY_POSIX_GRAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_POSIX_PRINT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_POSIX_PRINT = UNKNOWN;
    /** @cvalue UCHAR_POSIX_PRINT */
    #[\Since('8.4')]
    public const int PROPERTY_POSIX_PRINT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_POSIX_XDIGIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_POSIX_XDIGIT = UNKNOWN;
    /** @cvalue UCHAR_POSIX_XDIGIT */
    #[\Since('8.4')]
    public const int PROPERTY_POSIX_XDIGIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CASED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CASED = UNKNOWN;
    /** @cvalue UCHAR_CASED */
    #[\Since('8.4')]
    public const int PROPERTY_CASED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CASE_IGNORABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CASE_IGNORABLE = UNKNOWN;
    /** @cvalue UCHAR_CASE_IGNORABLE */
    #[\Since('8.4')]
    public const int PROPERTY_CASE_IGNORABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CHANGES_WHEN_LOWERCASED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CHANGES_WHEN_LOWERCASED = UNKNOWN;
    /** @cvalue UCHAR_CHANGES_WHEN_LOWERCASED */
    #[\Since('8.4')]
    public const int PROPERTY_CHANGES_WHEN_LOWERCASED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CHANGES_WHEN_UPPERCASED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CHANGES_WHEN_UPPERCASED = UNKNOWN;
    /** @cvalue UCHAR_CHANGES_WHEN_UPPERCASED */
    #[\Since('8.4')]
    public const int PROPERTY_CHANGES_WHEN_UPPERCASED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CHANGES_WHEN_TITLECASED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CHANGES_WHEN_TITLECASED = UNKNOWN;
    /** @cvalue UCHAR_CHANGES_WHEN_TITLECASED */
    #[\Since('8.4')]
    public const int PROPERTY_CHANGES_WHEN_TITLECASED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CHANGES_WHEN_CASEFOLDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CHANGES_WHEN_CASEFOLDED = UNKNOWN;
    /** @cvalue UCHAR_CHANGES_WHEN_CASEFOLDED */
    #[\Since('8.4')]
    public const int PROPERTY_CHANGES_WHEN_CASEFOLDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CHANGES_WHEN_CASEMAPPED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CHANGES_WHEN_CASEMAPPED = UNKNOWN;
    /** @cvalue UCHAR_CHANGES_WHEN_CASEMAPPED */
    #[\Since('8.4')]
    public const int PROPERTY_CHANGES_WHEN_CASEMAPPED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CHANGES_WHEN_NFKC_CASEFOLDED = UNKNOWN;
    /** @cvalue UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED */
    #[\Since('8.4')]
    public const int PROPERTY_CHANGES_WHEN_NFKC_CASEFOLDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BINARY_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BINARY_LIMIT = UNKNOWN;
    /** @cvalue UCHAR_BINARY_LIMIT */
    #[\Since('8.4')]
    public const int PROPERTY_BINARY_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BIDI_CLASS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BIDI_CLASS = UNKNOWN;
    /** @cvalue UCHAR_BIDI_CLASS */
    #[\Since('8.4')]
    public const int PROPERTY_BIDI_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_INT_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_INT_START = UNKNOWN;
    /** @cvalue UCHAR_INT_START */
    #[\Since('8.4')]
    public const int PROPERTY_INT_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BLOCK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BLOCK = UNKNOWN;
    /** @cvalue UCHAR_BLOCK */
    #[\Since('8.4')]
    public const int PROPERTY_BLOCK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CANONICAL_COMBINING_CLASS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CANONICAL_COMBINING_CLASS = UNKNOWN;
    /** @cvalue UCHAR_CANONICAL_COMBINING_CLASS */
    #[\Since('8.4')]
    public const int PROPERTY_CANONICAL_COMBINING_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DECOMPOSITION_TYPE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DECOMPOSITION_TYPE = UNKNOWN;
    /** @cvalue UCHAR_DECOMPOSITION_TYPE */
    #[\Since('8.4')]
    public const int PROPERTY_DECOMPOSITION_TYPE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_EAST_ASIAN_WIDTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_EAST_ASIAN_WIDTH = UNKNOWN;
    /** @cvalue UCHAR_EAST_ASIAN_WIDTH */
    #[\Since('8.4')]
    public const int PROPERTY_EAST_ASIAN_WIDTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_GENERAL_CATEGORY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_GENERAL_CATEGORY = UNKNOWN;
    /** @cvalue UCHAR_GENERAL_CATEGORY */
    #[\Since('8.4')]
    public const int PROPERTY_GENERAL_CATEGORY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_JOINING_GROUP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_JOINING_GROUP = UNKNOWN;
    /** @cvalue UCHAR_JOINING_GROUP */
    #[\Since('8.4')]
    public const int PROPERTY_JOINING_GROUP = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_JOINING_TYPE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_JOINING_TYPE = UNKNOWN;
    /** @cvalue UCHAR_JOINING_TYPE */
    #[\Since('8.4')]
    public const int PROPERTY_JOINING_TYPE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_LINE_BREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_LINE_BREAK = UNKNOWN;
    /** @cvalue UCHAR_LINE_BREAK */
    #[\Since('8.4')]
    public const int PROPERTY_LINE_BREAK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NUMERIC_TYPE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NUMERIC_TYPE = UNKNOWN;
    /** @cvalue UCHAR_NUMERIC_TYPE */
    #[\Since('8.4')]
    public const int PROPERTY_NUMERIC_TYPE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SCRIPT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SCRIPT = UNKNOWN;
    /** @cvalue UCHAR_SCRIPT */
    #[\Since('8.4')]
    public const int PROPERTY_SCRIPT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_HANGUL_SYLLABLE_TYPE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_HANGUL_SYLLABLE_TYPE = UNKNOWN;
    /** @cvalue UCHAR_HANGUL_SYLLABLE_TYPE */
    #[\Since('8.4')]
    public const int PROPERTY_HANGUL_SYLLABLE_TYPE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFD_QUICK_CHECK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFD_QUICK_CHECK = UNKNOWN;
    /** @cvalue UCHAR_NFD_QUICK_CHECK */
    #[\Since('8.4')]
    public const int PROPERTY_NFD_QUICK_CHECK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFKD_QUICK_CHECK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFKD_QUICK_CHECK = UNKNOWN;
    /** @cvalue UCHAR_NFKD_QUICK_CHECK */
    #[\Since('8.4')]
    public const int PROPERTY_NFKD_QUICK_CHECK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFC_QUICK_CHECK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFC_QUICK_CHECK = UNKNOWN;
    /** @cvalue UCHAR_NFC_QUICK_CHECK */
    #[\Since('8.4')]
    public const int PROPERTY_NFC_QUICK_CHECK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NFKC_QUICK_CHECK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NFKC_QUICK_CHECK = UNKNOWN;
    /** @cvalue UCHAR_NFKC_QUICK_CHECK */
    #[\Since('8.4')]
    public const int PROPERTY_NFKC_QUICK_CHECK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_LEAD_CANONICAL_COMBINING_CLASS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_LEAD_CANONICAL_COMBINING_CLASS = UNKNOWN;
    /** @cvalue UCHAR_LEAD_CANONICAL_COMBINING_CLASS */
    #[\Since('8.4')]
    public const int PROPERTY_LEAD_CANONICAL_COMBINING_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_TRAIL_CANONICAL_COMBINING_CLASS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_TRAIL_CANONICAL_COMBINING_CLASS = UNKNOWN;
    /** @cvalue UCHAR_TRAIL_CANONICAL_COMBINING_CLASS */
    #[\Since('8.4')]
    public const int PROPERTY_TRAIL_CANONICAL_COMBINING_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_GRAPHEME_CLUSTER_BREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_GRAPHEME_CLUSTER_BREAK = UNKNOWN;
    /** @cvalue UCHAR_GRAPHEME_CLUSTER_BREAK */
    #[\Since('8.4')]
    public const int PROPERTY_GRAPHEME_CLUSTER_BREAK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SENTENCE_BREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SENTENCE_BREAK = UNKNOWN;
    /** @cvalue UCHAR_SENTENCE_BREAK */
    #[\Since('8.4')]
    public const int PROPERTY_SENTENCE_BREAK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_WORD_BREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_WORD_BREAK = UNKNOWN;
    /** @cvalue UCHAR_WORD_BREAK */
    #[\Since('8.4')]
    public const int PROPERTY_WORD_BREAK = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @var int
     * @cvalue UCHAR_BIDI_PAIRED_BRACKET_TYPE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BIDI_PAIRED_BRACKET_TYPE = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /** @cvalue UCHAR_BIDI_PAIRED_BRACKET_TYPE */
    #[\Since('8.4')]
    public const int PROPERTY_BIDI_PAIRED_BRACKET_TYPE = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue UCHAR_INT_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_INT_LIMIT = UNKNOWN;
    #endif
    /** @cvalue UCHAR_INT_LIMIT */
    #[\Since('8.4')]
    public const int PROPERTY_INT_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_GENERAL_CATEGORY_MASK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_GENERAL_CATEGORY_MASK = UNKNOWN;
    /** @cvalue UCHAR_GENERAL_CATEGORY_MASK */
    #[\Since('8.4')]
    public const int PROPERTY_GENERAL_CATEGORY_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_MASK_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_MASK_START = UNKNOWN;
    /** @cvalue UCHAR_MASK_START */
    #[\Since('8.4')]
    public const int PROPERTY_MASK_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_MASK_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_MASK_LIMIT = UNKNOWN;
    /** @cvalue UCHAR_MASK_LIMIT */
    #[\Since('8.4')]
    public const int PROPERTY_MASK_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NUMERIC_VALUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NUMERIC_VALUE = UNKNOWN;
    /** @cvalue UCHAR_NUMERIC_VALUE */
    #[\Since('8.4')]
    public const int PROPERTY_NUMERIC_VALUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DOUBLE_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DOUBLE_START = UNKNOWN;
    /** @cvalue UCHAR_DOUBLE_START */
    #[\Since('8.4')]
    public const int PROPERTY_DOUBLE_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_DOUBLE_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_DOUBLE_LIMIT = UNKNOWN;
    /** @cvalue UCHAR_DOUBLE_LIMIT */
    #[\Since('8.4')]
    public const int PROPERTY_DOUBLE_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_AGE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_AGE = UNKNOWN;
    /** @cvalue UCHAR_AGE */
    #[\Since('8.4')]
    public const int PROPERTY_AGE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_STRING_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_STRING_START = UNKNOWN;
    /** @cvalue UCHAR_STRING_START */
    #[\Since('8.4')]
    public const int PROPERTY_STRING_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_BIDI_MIRRORING_GLYPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BIDI_MIRRORING_GLYPH = UNKNOWN;
    /** @cvalue UCHAR_BIDI_MIRRORING_GLYPH */
    #[\Since('8.4')]
    public const int PROPERTY_BIDI_MIRRORING_GLYPH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_CASE_FOLDING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_CASE_FOLDING = UNKNOWN;
    /** @cvalue UCHAR_CASE_FOLDING */
    #[\Since('8.4')]
    public const int PROPERTY_CASE_FOLDING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_ISO_COMMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_ISO_COMMENT = UNKNOWN;
    /** @cvalue UCHAR_ISO_COMMENT */
    #[\Since('8.4')]
    public const int PROPERTY_ISO_COMMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_LOWERCASE_MAPPING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_LOWERCASE_MAPPING = UNKNOWN;
    /** @cvalue UCHAR_LOWERCASE_MAPPING */
    #[\Since('8.4')]
    public const int PROPERTY_LOWERCASE_MAPPING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NAME = UNKNOWN;
    /** @cvalue UCHAR_NAME */
    #[\Since('8.4')]
    public const int PROPERTY_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SIMPLE_CASE_FOLDING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SIMPLE_CASE_FOLDING = UNKNOWN;
    /** @cvalue UCHAR_SIMPLE_CASE_FOLDING */
    #[\Since('8.4')]
    public const int PROPERTY_SIMPLE_CASE_FOLDING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SIMPLE_LOWERCASE_MAPPING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SIMPLE_LOWERCASE_MAPPING = UNKNOWN;
    /** @cvalue UCHAR_SIMPLE_LOWERCASE_MAPPING */
    #[\Since('8.4')]
    public const int PROPERTY_SIMPLE_LOWERCASE_MAPPING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SIMPLE_TITLECASE_MAPPING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SIMPLE_TITLECASE_MAPPING = UNKNOWN;
    /** @cvalue UCHAR_SIMPLE_TITLECASE_MAPPING */
    #[\Since('8.4')]
    public const int PROPERTY_SIMPLE_TITLECASE_MAPPING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SIMPLE_UPPERCASE_MAPPING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SIMPLE_UPPERCASE_MAPPING = UNKNOWN;
    /** @cvalue UCHAR_SIMPLE_UPPERCASE_MAPPING */
    #[\Since('8.4')]
    public const int PROPERTY_SIMPLE_UPPERCASE_MAPPING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_TITLECASE_MAPPING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_TITLECASE_MAPPING = UNKNOWN;
    /** @cvalue UCHAR_TITLECASE_MAPPING */
    #[\Since('8.4')]
    public const int PROPERTY_TITLECASE_MAPPING = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_UNICODE_1_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_UNICODE_1_NAME = UNKNOWN;
    /** @cvalue UCHAR_UNICODE_1_NAME */
    #[\Since('8.4')]
    public const int PROPERTY_UNICODE_1_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_UPPERCASE_MAPPING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_UPPERCASE_MAPPING = UNKNOWN;
    /** @cvalue UCHAR_UPPERCASE_MAPPING */
    #[\Since('8.4')]
    public const int PROPERTY_UPPERCASE_MAPPING = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @var int
     * @cvalue UCHAR_BIDI_PAIRED_BRACKET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_BIDI_PAIRED_BRACKET = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /** @cvalue UCHAR_BIDI_PAIRED_BRACKET */
    #[\Since('8.4')]
    public const int PROPERTY_BIDI_PAIRED_BRACKET = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue UCHAR_STRING_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_STRING_LIMIT = UNKNOWN;
    #endif
    /** @cvalue UCHAR_STRING_LIMIT */
    #[\Since('8.4')]
    public const int PROPERTY_STRING_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_SCRIPT_EXTENSIONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_SCRIPT_EXTENSIONS = UNKNOWN;
    /** @cvalue UCHAR_SCRIPT_EXTENSIONS */
    #[\Since('8.4')]
    public const int PROPERTY_SCRIPT_EXTENSIONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_OTHER_PROPERTY_START
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_OTHER_PROPERTY_START = UNKNOWN;
    /** @cvalue UCHAR_OTHER_PROPERTY_START */
    #[\Since('8.4')]
    public const int PROPERTY_OTHER_PROPERTY_START = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_OTHER_PROPERTY_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_OTHER_PROPERTY_LIMIT = UNKNOWN;
    /** @cvalue UCHAR_OTHER_PROPERTY_LIMIT */
    #[\Since('8.4')]
    public const int PROPERTY_OTHER_PROPERTY_LIMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCHAR_INVALID_CODE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_INVALID_CODE = UNKNOWN;
    /** @cvalue UCHAR_INVALID_CODE */
    #[\Since('8.4')]
    public const int PROPERTY_INVALID_CODE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_UNASSIGNED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_UNASSIGNED = UNKNOWN;
    /** @cvalue U_UNASSIGNED */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_UNASSIGNED = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GENERAL_OTHER_TYPES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_GENERAL_OTHER_TYPES = UNKNOWN;
    /** @cvalue U_GENERAL_OTHER_TYPES */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_GENERAL_OTHER_TYPES = UNKNOWN;
    /**
     * @var int
     * @cvalue U_UPPERCASE_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_UPPERCASE_LETTER = UNKNOWN;
    /** @cvalue U_UPPERCASE_LETTER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_UPPERCASE_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LOWERCASE_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_LOWERCASE_LETTER = UNKNOWN;
    /** @cvalue U_LOWERCASE_LETTER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_LOWERCASE_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_TITLECASE_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_TITLECASE_LETTER = UNKNOWN;
    /** @cvalue U_TITLECASE_LETTER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_TITLECASE_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_MODIFIER_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_MODIFIER_LETTER = UNKNOWN;
    /** @cvalue U_MODIFIER_LETTER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_MODIFIER_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_OTHER_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_OTHER_LETTER = UNKNOWN;
    /** @cvalue U_OTHER_LETTER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_OTHER_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_NON_SPACING_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_NON_SPACING_MARK = UNKNOWN;
    /** @cvalue U_NON_SPACING_MARK */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_NON_SPACING_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_ENCLOSING_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_ENCLOSING_MARK = UNKNOWN;
    /** @cvalue U_ENCLOSING_MARK */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_ENCLOSING_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_COMBINING_SPACING_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_COMBINING_SPACING_MARK = UNKNOWN;
    /** @cvalue U_COMBINING_SPACING_MARK */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_COMBINING_SPACING_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DECIMAL_DIGIT_NUMBER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_DECIMAL_DIGIT_NUMBER = UNKNOWN;
    /** @cvalue U_DECIMAL_DIGIT_NUMBER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_DECIMAL_DIGIT_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LETTER_NUMBER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_LETTER_NUMBER = UNKNOWN;
    /** @cvalue U_LETTER_NUMBER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_LETTER_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_OTHER_NUMBER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_OTHER_NUMBER = UNKNOWN;
    /** @cvalue U_OTHER_NUMBER */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_OTHER_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SPACE_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_SPACE_SEPARATOR = UNKNOWN;
    /** @cvalue U_SPACE_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_SPACE_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LINE_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_LINE_SEPARATOR = UNKNOWN;
    /** @cvalue U_LINE_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_LINE_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_PARAGRAPH_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_PARAGRAPH_SEPARATOR = UNKNOWN;
    /** @cvalue U_PARAGRAPH_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_PARAGRAPH_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_CONTROL_CHAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_CONTROL_CHAR = UNKNOWN;
    /** @cvalue U_CONTROL_CHAR */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_CONTROL_CHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_FORMAT_CHAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_FORMAT_CHAR = UNKNOWN;
    /** @cvalue U_FORMAT_CHAR */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_FORMAT_CHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_PRIVATE_USE_CHAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_PRIVATE_USE_CHAR = UNKNOWN;
    /** @cvalue U_PRIVATE_USE_CHAR */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_PRIVATE_USE_CHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SURROGATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_SURROGATE = UNKNOWN;
    /** @cvalue U_SURROGATE */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_SURROGATE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DASH_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_DASH_PUNCTUATION = UNKNOWN;
    /** @cvalue U_DASH_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_DASH_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_START_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_START_PUNCTUATION = UNKNOWN;
    /** @cvalue U_START_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_START_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_END_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_END_PUNCTUATION = UNKNOWN;
    /** @cvalue U_END_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_END_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_CONNECTOR_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_CONNECTOR_PUNCTUATION = UNKNOWN;
    /** @cvalue U_CONNECTOR_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_CONNECTOR_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_OTHER_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_OTHER_PUNCTUATION = UNKNOWN;
    /** @cvalue U_OTHER_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_OTHER_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_MATH_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_MATH_SYMBOL = UNKNOWN;
    /** @cvalue U_MATH_SYMBOL */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_MATH_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_CURRENCY_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_CURRENCY_SYMBOL = UNKNOWN;
    /** @cvalue U_CURRENCY_SYMBOL */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_CURRENCY_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_MODIFIER_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_MODIFIER_SYMBOL = UNKNOWN;
    /** @cvalue U_MODIFIER_SYMBOL */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_MODIFIER_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_OTHER_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_OTHER_SYMBOL = UNKNOWN;
    /** @cvalue U_OTHER_SYMBOL */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_OTHER_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_INITIAL_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_INITIAL_PUNCTUATION = UNKNOWN;
    /** @cvalue U_INITIAL_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_INITIAL_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_FINAL_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_FINAL_PUNCTUATION = UNKNOWN;
    /** @cvalue U_FINAL_PUNCTUATION */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_FINAL_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_CHAR_CATEGORY_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_CATEGORY_CHAR_CATEGORY_COUNT = UNKNOWN;
    /** @cvalue U_CHAR_CATEGORY_COUNT */
    #[\Since('8.4')]
    public const int CHAR_CATEGORY_CHAR_CATEGORY_COUNT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LEFT_TO_RIGHT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_LEFT_TO_RIGHT = UNKNOWN;
    /** @cvalue U_LEFT_TO_RIGHT */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_LEFT_TO_RIGHT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_RIGHT_TO_LEFT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_RIGHT_TO_LEFT = UNKNOWN;
    /** @cvalue U_RIGHT_TO_LEFT */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_RIGHT_TO_LEFT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EUROPEAN_NUMBER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_EUROPEAN_NUMBER = UNKNOWN;
    /** @cvalue U_EUROPEAN_NUMBER */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_EUROPEAN_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EUROPEAN_NUMBER_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_EUROPEAN_NUMBER_SEPARATOR = UNKNOWN;
    /** @cvalue U_EUROPEAN_NUMBER_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_EUROPEAN_NUMBER_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EUROPEAN_NUMBER_TERMINATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_EUROPEAN_NUMBER_TERMINATOR = UNKNOWN;
    /** @cvalue U_EUROPEAN_NUMBER_TERMINATOR */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_EUROPEAN_NUMBER_TERMINATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_ARABIC_NUMBER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_ARABIC_NUMBER = UNKNOWN;
    /** @cvalue U_ARABIC_NUMBER */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_ARABIC_NUMBER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_COMMON_NUMBER_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_COMMON_NUMBER_SEPARATOR = UNKNOWN;
    /** @cvalue U_COMMON_NUMBER_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_COMMON_NUMBER_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_BLOCK_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_BLOCK_SEPARATOR = UNKNOWN;
    /** @cvalue U_BLOCK_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_BLOCK_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SEGMENT_SEPARATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_SEGMENT_SEPARATOR = UNKNOWN;
    /** @cvalue U_SEGMENT_SEPARATOR */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_SEGMENT_SEPARATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WHITE_SPACE_NEUTRAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_WHITE_SPACE_NEUTRAL = UNKNOWN;
    /** @cvalue U_WHITE_SPACE_NEUTRAL */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_WHITE_SPACE_NEUTRAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_OTHER_NEUTRAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_OTHER_NEUTRAL = UNKNOWN;
    /** @cvalue U_OTHER_NEUTRAL */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_OTHER_NEUTRAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LEFT_TO_RIGHT_EMBEDDING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_LEFT_TO_RIGHT_EMBEDDING = UNKNOWN;
    /** @cvalue U_LEFT_TO_RIGHT_EMBEDDING */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_LEFT_TO_RIGHT_EMBEDDING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LEFT_TO_RIGHT_OVERRIDE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_LEFT_TO_RIGHT_OVERRIDE = UNKNOWN;
    /** @cvalue U_LEFT_TO_RIGHT_OVERRIDE */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_LEFT_TO_RIGHT_OVERRIDE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_RIGHT_TO_LEFT_ARABIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_RIGHT_TO_LEFT_ARABIC = UNKNOWN;
    /** @cvalue U_RIGHT_TO_LEFT_ARABIC */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_RIGHT_TO_LEFT_ARABIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_RIGHT_TO_LEFT_EMBEDDING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_RIGHT_TO_LEFT_EMBEDDING = UNKNOWN;
    /** @cvalue U_RIGHT_TO_LEFT_EMBEDDING */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_RIGHT_TO_LEFT_EMBEDDING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_RIGHT_TO_LEFT_OVERRIDE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_RIGHT_TO_LEFT_OVERRIDE = UNKNOWN;
    /** @cvalue U_RIGHT_TO_LEFT_OVERRIDE */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_RIGHT_TO_LEFT_OVERRIDE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_POP_DIRECTIONAL_FORMAT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_POP_DIRECTIONAL_FORMAT = UNKNOWN;
    /** @cvalue U_POP_DIRECTIONAL_FORMAT */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_POP_DIRECTIONAL_FORMAT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DIR_NON_SPACING_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_DIR_NON_SPACING_MARK = UNKNOWN;
    /** @cvalue U_DIR_NON_SPACING_MARK */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_DIR_NON_SPACING_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_BOUNDARY_NEUTRAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_BOUNDARY_NEUTRAL = UNKNOWN;
    /** @cvalue U_BOUNDARY_NEUTRAL */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_BOUNDARY_NEUTRAL = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @var int
     * @cvalue U_FIRST_STRONG_ISOLATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_FIRST_STRONG_ISOLATE = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /** @cvalue U_FIRST_STRONG_ISOLATE */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_FIRST_STRONG_ISOLATE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LEFT_TO_RIGHT_ISOLATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_LEFT_TO_RIGHT_ISOLATE = UNKNOWN;
    /** @cvalue U_LEFT_TO_RIGHT_ISOLATE */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_LEFT_TO_RIGHT_ISOLATE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_RIGHT_TO_LEFT_ISOLATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_RIGHT_TO_LEFT_ISOLATE = UNKNOWN;
    /** @cvalue U_RIGHT_TO_LEFT_ISOLATE */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_RIGHT_TO_LEFT_ISOLATE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_POP_DIRECTIONAL_ISOLATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_POP_DIRECTIONAL_ISOLATE = UNKNOWN;
    /** @cvalue U_POP_DIRECTIONAL_ISOLATE */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_POP_DIRECTIONAL_ISOLATE = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue U_CHAR_DIRECTION_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_DIRECTION_CHAR_DIRECTION_COUNT = UNKNOWN;
    #endif
    /** @cvalue U_CHAR_DIRECTION_COUNT */
    #[\Since('8.4')]
    public const int CHAR_DIRECTION_CHAR_DIRECTION_COUNT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_NO_BLOCK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_NO_BLOCK = UNKNOWN;
    /** @cvalue UBLOCK_NO_BLOCK */
    #[\Since('8.4')]
    public const int BLOCK_CODE_NO_BLOCK = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BASIC_LATIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BASIC_LATIN = UNKNOWN;
    /** @cvalue UBLOCK_BASIC_LATIN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BASIC_LATIN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_1_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_1_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_1_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_1_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_EXTENDED_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_EXTENDED_A = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_EXTENDED_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_EXTENDED_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_EXTENDED_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_EXTENDED_B = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_EXTENDED_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_EXTENDED_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_IPA_EXTENSIONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_IPA_EXTENSIONS = UNKNOWN;
    /** @cvalue UBLOCK_IPA_EXTENSIONS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_IPA_EXTENSIONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SPACING_MODIFIER_LETTERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SPACING_MODIFIER_LETTERS = UNKNOWN;
    /** @cvalue UBLOCK_SPACING_MODIFIER_LETTERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SPACING_MODIFIER_LETTERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COMBINING_DIACRITICAL_MARKS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COMBINING_DIACRITICAL_MARKS = UNKNOWN;
    /** @cvalue UBLOCK_COMBINING_DIACRITICAL_MARKS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COMBINING_DIACRITICAL_MARKS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GREEK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GREEK = UNKNOWN;
    /** @cvalue UBLOCK_GREEK */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GREEK = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CYRILLIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CYRILLIC = UNKNOWN;
    /** @cvalue UBLOCK_CYRILLIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CYRILLIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARMENIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARMENIAN = UNKNOWN;
    /** @cvalue UBLOCK_ARMENIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARMENIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HEBREW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HEBREW = UNKNOWN;
    /** @cvalue UBLOCK_HEBREW */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HEBREW = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARABIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARABIC = UNKNOWN;
    /** @cvalue UBLOCK_ARABIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARABIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SYRIAC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SYRIAC = UNKNOWN;
    /** @cvalue UBLOCK_SYRIAC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SYRIAC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_THAANA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_THAANA = UNKNOWN;
    /** @cvalue UBLOCK_THAANA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_THAANA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_DEVANAGARI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_DEVANAGARI = UNKNOWN;
    /** @cvalue UBLOCK_DEVANAGARI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_DEVANAGARI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BENGALI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BENGALI = UNKNOWN;
    /** @cvalue UBLOCK_BENGALI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BENGALI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GURMUKHI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GURMUKHI = UNKNOWN;
    /** @cvalue UBLOCK_GURMUKHI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GURMUKHI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GUJARATI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GUJARATI = UNKNOWN;
    /** @cvalue UBLOCK_GUJARATI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GUJARATI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ORIYA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ORIYA = UNKNOWN;
    /** @cvalue UBLOCK_ORIYA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ORIYA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAMIL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAMIL = UNKNOWN;
    /** @cvalue UBLOCK_TAMIL */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAMIL = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TELUGU
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TELUGU = UNKNOWN;
    /** @cvalue UBLOCK_TELUGU */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TELUGU = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KANNADA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KANNADA = UNKNOWN;
    /** @cvalue UBLOCK_KANNADA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KANNADA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MALAYALAM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MALAYALAM = UNKNOWN;
    /** @cvalue UBLOCK_MALAYALAM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MALAYALAM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SINHALA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SINHALA = UNKNOWN;
    /** @cvalue UBLOCK_SINHALA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SINHALA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_THAI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_THAI = UNKNOWN;
    /** @cvalue UBLOCK_THAI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_THAI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LAO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LAO = UNKNOWN;
    /** @cvalue UBLOCK_LAO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LAO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TIBETAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TIBETAN = UNKNOWN;
    /** @cvalue UBLOCK_TIBETAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TIBETAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MYANMAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MYANMAR = UNKNOWN;
    /** @cvalue UBLOCK_MYANMAR */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MYANMAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GEORGIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GEORGIAN = UNKNOWN;
    /** @cvalue UBLOCK_GEORGIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GEORGIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HANGUL_JAMO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HANGUL_JAMO = UNKNOWN;
    /** @cvalue UBLOCK_HANGUL_JAMO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HANGUL_JAMO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ETHIOPIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ETHIOPIC = UNKNOWN;
    /** @cvalue UBLOCK_ETHIOPIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ETHIOPIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CHEROKEE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CHEROKEE = UNKNOWN;
    /** @cvalue UBLOCK_CHEROKEE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CHEROKEE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS = UNKNOWN;
    /** @cvalue UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OGHAM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OGHAM = UNKNOWN;
    /** @cvalue UBLOCK_OGHAM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OGHAM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_RUNIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_RUNIC = UNKNOWN;
    /** @cvalue UBLOCK_RUNIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_RUNIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KHMER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KHMER = UNKNOWN;
    /** @cvalue UBLOCK_KHMER */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KHMER = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MONGOLIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MONGOLIAN = UNKNOWN;
    /** @cvalue UBLOCK_MONGOLIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MONGOLIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_EXTENDED_ADDITIONAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_EXTENDED_ADDITIONAL = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_EXTENDED_ADDITIONAL */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_EXTENDED_ADDITIONAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GREEK_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GREEK_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_GREEK_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GREEK_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GENERAL_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GENERAL_PUNCTUATION = UNKNOWN;
    /** @cvalue UBLOCK_GENERAL_PUNCTUATION */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GENERAL_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPERSCRIPTS_AND_SUBSCRIPTS = UNKNOWN;
    /** @cvalue UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPERSCRIPTS_AND_SUBSCRIPTS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CURRENCY_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CURRENCY_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_CURRENCY_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CURRENCY_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COMBINING_MARKS_FOR_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COMBINING_MARKS_FOR_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_COMBINING_MARKS_FOR_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COMBINING_MARKS_FOR_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LETTERLIKE_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LETTERLIKE_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_LETTERLIKE_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LETTERLIKE_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_NUMBER_FORMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_NUMBER_FORMS = UNKNOWN;
    /** @cvalue UBLOCK_NUMBER_FORMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_NUMBER_FORMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARROWS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARROWS = UNKNOWN;
    /** @cvalue UBLOCK_ARROWS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARROWS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MATHEMATICAL_OPERATORS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MATHEMATICAL_OPERATORS = UNKNOWN;
    /** @cvalue UBLOCK_MATHEMATICAL_OPERATORS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MATHEMATICAL_OPERATORS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MISCELLANEOUS_TECHNICAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MISCELLANEOUS_TECHNICAL = UNKNOWN;
    /** @cvalue UBLOCK_MISCELLANEOUS_TECHNICAL */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MISCELLANEOUS_TECHNICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CONTROL_PICTURES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CONTROL_PICTURES = UNKNOWN;
    /** @cvalue UBLOCK_CONTROL_PICTURES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CONTROL_PICTURES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OPTICAL_CHARACTER_RECOGNITION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OPTICAL_CHARACTER_RECOGNITION = UNKNOWN;
    /** @cvalue UBLOCK_OPTICAL_CHARACTER_RECOGNITION */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OPTICAL_CHARACTER_RECOGNITION = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ENCLOSED_ALPHANUMERICS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ENCLOSED_ALPHANUMERICS = UNKNOWN;
    /** @cvalue UBLOCK_ENCLOSED_ALPHANUMERICS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ENCLOSED_ALPHANUMERICS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BOX_DRAWING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BOX_DRAWING = UNKNOWN;
    /** @cvalue UBLOCK_BOX_DRAWING */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BOX_DRAWING = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BLOCK_ELEMENTS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BLOCK_ELEMENTS = UNKNOWN;
    /** @cvalue UBLOCK_BLOCK_ELEMENTS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BLOCK_ELEMENTS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GEOMETRIC_SHAPES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GEOMETRIC_SHAPES = UNKNOWN;
    /** @cvalue UBLOCK_GEOMETRIC_SHAPES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GEOMETRIC_SHAPES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MISCELLANEOUS_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MISCELLANEOUS_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_MISCELLANEOUS_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MISCELLANEOUS_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_DINGBATS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_DINGBATS = UNKNOWN;
    /** @cvalue UBLOCK_DINGBATS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_DINGBATS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BRAILLE_PATTERNS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BRAILLE_PATTERNS = UNKNOWN;
    /** @cvalue UBLOCK_BRAILLE_PATTERNS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BRAILLE_PATTERNS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_RADICALS_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_RADICALS_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_CJK_RADICALS_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_RADICALS_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KANGXI_RADICALS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KANGXI_RADICALS = UNKNOWN;
    /** @cvalue UBLOCK_KANGXI_RADICALS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KANGXI_RADICALS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_IDEOGRAPHIC_DESCRIPTION_CHARACTERS = UNKNOWN;
    /** @cvalue UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_IDEOGRAPHIC_DESCRIPTION_CHARACTERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_SYMBOLS_AND_PUNCTUATION = UNKNOWN;
    /** @cvalue UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_SYMBOLS_AND_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HIRAGANA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HIRAGANA = UNKNOWN;
    /** @cvalue UBLOCK_HIRAGANA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HIRAGANA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KATAKANA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KATAKANA = UNKNOWN;
    /** @cvalue UBLOCK_KATAKANA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KATAKANA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BOPOMOFO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BOPOMOFO = UNKNOWN;
    /** @cvalue UBLOCK_BOPOMOFO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BOPOMOFO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HANGUL_COMPATIBILITY_JAMO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HANGUL_COMPATIBILITY_JAMO = UNKNOWN;
    /** @cvalue UBLOCK_HANGUL_COMPATIBILITY_JAMO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HANGUL_COMPATIBILITY_JAMO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KANBUN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KANBUN = UNKNOWN;
    /** @cvalue UBLOCK_KANBUN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KANBUN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BOPOMOFO_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BOPOMOFO_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_BOPOMOFO_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BOPOMOFO_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ENCLOSED_CJK_LETTERS_AND_MONTHS = UNKNOWN;
    /** @cvalue UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ENCLOSED_CJK_LETTERS_AND_MONTHS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_COMPATIBILITY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_COMPATIBILITY = UNKNOWN;
    /** @cvalue UBLOCK_CJK_COMPATIBILITY */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_COMPATIBILITY = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A = UNKNOWN;
    /** @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS = UNKNOWN;
    /** @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_YI_SYLLABLES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_YI_SYLLABLES = UNKNOWN;
    /** @cvalue UBLOCK_YI_SYLLABLES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_YI_SYLLABLES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_YI_RADICALS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_YI_RADICALS = UNKNOWN;
    /** @cvalue UBLOCK_YI_RADICALS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_YI_RADICALS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HANGUL_SYLLABLES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HANGUL_SYLLABLES = UNKNOWN;
    /** @cvalue UBLOCK_HANGUL_SYLLABLES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HANGUL_SYLLABLES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HIGH_SURROGATES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HIGH_SURROGATES = UNKNOWN;
    /** @cvalue UBLOCK_HIGH_SURROGATES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HIGH_SURROGATES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HIGH_PRIVATE_USE_SURROGATES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HIGH_PRIVATE_USE_SURROGATES = UNKNOWN;
    /** @cvalue UBLOCK_HIGH_PRIVATE_USE_SURROGATES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HIGH_PRIVATE_USE_SURROGATES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LOW_SURROGATES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LOW_SURROGATES = UNKNOWN;
    /** @cvalue UBLOCK_LOW_SURROGATES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LOW_SURROGATES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PRIVATE_USE_AREA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PRIVATE_USE_AREA = UNKNOWN;
    /** @cvalue UBLOCK_PRIVATE_USE_AREA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PRIVATE_USE_AREA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PRIVATE_USE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PRIVATE_USE = UNKNOWN;
    /** @cvalue UBLOCK_PRIVATE_USE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PRIVATE_USE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_COMPATIBILITY_IDEOGRAPHS = UNKNOWN;
    /** @cvalue UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_COMPATIBILITY_IDEOGRAPHS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ALPHABETIC_PRESENTATION_FORMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ALPHABETIC_PRESENTATION_FORMS = UNKNOWN;
    /** @cvalue UBLOCK_ALPHABETIC_PRESENTATION_FORMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ALPHABETIC_PRESENTATION_FORMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARABIC_PRESENTATION_FORMS_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARABIC_PRESENTATION_FORMS_A = UNKNOWN;
    /** @cvalue UBLOCK_ARABIC_PRESENTATION_FORMS_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARABIC_PRESENTATION_FORMS_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COMBINING_HALF_MARKS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COMBINING_HALF_MARKS = UNKNOWN;
    /** @cvalue UBLOCK_COMBINING_HALF_MARKS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COMBINING_HALF_MARKS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_COMPATIBILITY_FORMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_COMPATIBILITY_FORMS = UNKNOWN;
    /** @cvalue UBLOCK_CJK_COMPATIBILITY_FORMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_COMPATIBILITY_FORMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SMALL_FORM_VARIANTS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SMALL_FORM_VARIANTS = UNKNOWN;
    /** @cvalue UBLOCK_SMALL_FORM_VARIANTS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SMALL_FORM_VARIANTS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARABIC_PRESENTATION_FORMS_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARABIC_PRESENTATION_FORMS_B = UNKNOWN;
    /** @cvalue UBLOCK_ARABIC_PRESENTATION_FORMS_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARABIC_PRESENTATION_FORMS_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SPECIALS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SPECIALS = UNKNOWN;
    /** @cvalue UBLOCK_SPECIALS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SPECIALS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HALFWIDTH_AND_FULLWIDTH_FORMS = UNKNOWN;
    /** @cvalue UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HALFWIDTH_AND_FULLWIDTH_FORMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OLD_ITALIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OLD_ITALIC = UNKNOWN;
    /** @cvalue UBLOCK_OLD_ITALIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OLD_ITALIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GOTHIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GOTHIC = UNKNOWN;
    /** @cvalue UBLOCK_GOTHIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GOTHIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_DESERET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_DESERET = UNKNOWN;
    /** @cvalue UBLOCK_DESERET */
    #[\Since('8.4')]
    public const int BLOCK_CODE_DESERET = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BYZANTINE_MUSICAL_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BYZANTINE_MUSICAL_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_BYZANTINE_MUSICAL_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BYZANTINE_MUSICAL_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MUSICAL_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MUSICAL_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_MUSICAL_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MUSICAL_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MATHEMATICAL_ALPHANUMERIC_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MATHEMATICAL_ALPHANUMERIC_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B = UNKNOWN;
    /** @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAGS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAGS = UNKNOWN;
    /** @cvalue UBLOCK_TAGS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAGS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CYRILLIC_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CYRILLIC_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_CYRILLIC_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CYRILLIC_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CYRILLIC_SUPPLEMENTARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CYRILLIC_SUPPLEMENTARY = UNKNOWN;
    /** @cvalue UBLOCK_CYRILLIC_SUPPLEMENTARY */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CYRILLIC_SUPPLEMENTARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAGALOG
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAGALOG = UNKNOWN;
    /** @cvalue UBLOCK_TAGALOG */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAGALOG = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HANUNOO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HANUNOO = UNKNOWN;
    /** @cvalue UBLOCK_HANUNOO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HANUNOO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BUHID
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BUHID = UNKNOWN;
    /** @cvalue UBLOCK_BUHID */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BUHID = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAGBANWA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAGBANWA = UNKNOWN;
    /** @cvalue UBLOCK_TAGBANWA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAGBANWA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A = UNKNOWN;
    /** @cvalue UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTAL_ARROWS_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTAL_ARROWS_A = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTAL_ARROWS_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTAL_ARROWS_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTAL_ARROWS_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTAL_ARROWS_B = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTAL_ARROWS_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTAL_ARROWS_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B = UNKNOWN;
    /** @cvalue UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTAL_MATHEMATICAL_OPERATORS = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTAL_MATHEMATICAL_OPERATORS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KATAKANA_PHONETIC_EXTENSIONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KATAKANA_PHONETIC_EXTENSIONS = UNKNOWN;
    /** @cvalue UBLOCK_KATAKANA_PHONETIC_EXTENSIONS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KATAKANA_PHONETIC_EXTENSIONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_VARIATION_SELECTORS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_VARIATION_SELECTORS = UNKNOWN;
    /** @cvalue UBLOCK_VARIATION_SELECTORS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_VARIATION_SELECTORS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTARY_PRIVATE_USE_AREA_A = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTARY_PRIVATE_USE_AREA_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTARY_PRIVATE_USE_AREA_B = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTARY_PRIVATE_USE_AREA_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LIMBU
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LIMBU = UNKNOWN;
    /** @cvalue UBLOCK_LIMBU */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LIMBU = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAI_LE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAI_LE = UNKNOWN;
    /** @cvalue UBLOCK_TAI_LE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAI_LE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KHMER_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KHMER_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_KHMER_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KHMER_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PHONETIC_EXTENSIONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PHONETIC_EXTENSIONS = UNKNOWN;
    /** @cvalue UBLOCK_PHONETIC_EXTENSIONS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PHONETIC_EXTENSIONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MISCELLANEOUS_SYMBOLS_AND_ARROWS = UNKNOWN;
    /** @cvalue UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MISCELLANEOUS_SYMBOLS_AND_ARROWS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_YIJING_HEXAGRAM_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_YIJING_HEXAGRAM_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_YIJING_HEXAGRAM_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_YIJING_HEXAGRAM_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LINEAR_B_SYLLABARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LINEAR_B_SYLLABARY = UNKNOWN;
    /** @cvalue UBLOCK_LINEAR_B_SYLLABARY */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LINEAR_B_SYLLABARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LINEAR_B_IDEOGRAMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LINEAR_B_IDEOGRAMS = UNKNOWN;
    /** @cvalue UBLOCK_LINEAR_B_IDEOGRAMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LINEAR_B_IDEOGRAMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_AEGEAN_NUMBERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_AEGEAN_NUMBERS = UNKNOWN;
    /** @cvalue UBLOCK_AEGEAN_NUMBERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_AEGEAN_NUMBERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_UGARITIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_UGARITIC = UNKNOWN;
    /** @cvalue UBLOCK_UGARITIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_UGARITIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SHAVIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SHAVIAN = UNKNOWN;
    /** @cvalue UBLOCK_SHAVIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SHAVIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OSMANYA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OSMANYA = UNKNOWN;
    /** @cvalue UBLOCK_OSMANYA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OSMANYA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CYPRIOT_SYLLABARY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CYPRIOT_SYLLABARY = UNKNOWN;
    /** @cvalue UBLOCK_CYPRIOT_SYLLABARY */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CYPRIOT_SYLLABARY = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAI_XUAN_JING_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAI_XUAN_JING_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_TAI_XUAN_JING_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAI_XUAN_JING_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_VARIATION_SELECTORS_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_VARIATION_SELECTORS_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_VARIATION_SELECTORS_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_VARIATION_SELECTORS_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ANCIENT_GREEK_MUSICAL_NOTATION = UNKNOWN;
    /** @cvalue UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ANCIENT_GREEK_MUSICAL_NOTATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ANCIENT_GREEK_NUMBERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ANCIENT_GREEK_NUMBERS = UNKNOWN;
    /** @cvalue UBLOCK_ANCIENT_GREEK_NUMBERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ANCIENT_GREEK_NUMBERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARABIC_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARABIC_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_ARABIC_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARABIC_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BUGINESE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BUGINESE = UNKNOWN;
    /** @cvalue UBLOCK_BUGINESE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BUGINESE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_STROKES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_STROKES = UNKNOWN;
    /** @cvalue UBLOCK_CJK_STROKES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_STROKES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COPTIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COPTIC = UNKNOWN;
    /** @cvalue UBLOCK_COPTIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COPTIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ETHIOPIC_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ETHIOPIC_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_ETHIOPIC_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ETHIOPIC_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ETHIOPIC_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ETHIOPIC_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_ETHIOPIC_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ETHIOPIC_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GEORGIAN_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GEORGIAN_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_GEORGIAN_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GEORGIAN_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GLAGOLITIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GLAGOLITIC = UNKNOWN;
    /** @cvalue UBLOCK_GLAGOLITIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GLAGOLITIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KHAROSHTHI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KHAROSHTHI = UNKNOWN;
    /** @cvalue UBLOCK_KHAROSHTHI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KHAROSHTHI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MODIFIER_TONE_LETTERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MODIFIER_TONE_LETTERS = UNKNOWN;
    /** @cvalue UBLOCK_MODIFIER_TONE_LETTERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MODIFIER_TONE_LETTERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_NEW_TAI_LUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_NEW_TAI_LUE = UNKNOWN;
    /** @cvalue UBLOCK_NEW_TAI_LUE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_NEW_TAI_LUE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OLD_PERSIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OLD_PERSIAN = UNKNOWN;
    /** @cvalue UBLOCK_OLD_PERSIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OLD_PERSIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PHONETIC_EXTENSIONS_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PHONETIC_EXTENSIONS_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTAL_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTAL_PUNCTUATION = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTAL_PUNCTUATION */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTAL_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SYLOTI_NAGRI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SYLOTI_NAGRI = UNKNOWN;
    /** @cvalue UBLOCK_SYLOTI_NAGRI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SYLOTI_NAGRI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TIFINAGH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TIFINAGH = UNKNOWN;
    /** @cvalue UBLOCK_TIFINAGH */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TIFINAGH = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_VERTICAL_FORMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_VERTICAL_FORMS = UNKNOWN;
    /** @cvalue UBLOCK_VERTICAL_FORMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_VERTICAL_FORMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_NKO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_NKO = UNKNOWN;
    /** @cvalue UBLOCK_NKO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_NKO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BALINESE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BALINESE = UNKNOWN;
    /** @cvalue UBLOCK_BALINESE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BALINESE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_EXTENDED_C
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_EXTENDED_C = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_EXTENDED_C */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_EXTENDED_C = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_EXTENDED_D
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_EXTENDED_D = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_EXTENDED_D */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_EXTENDED_D = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PHAGS_PA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PHAGS_PA = UNKNOWN;
    /** @cvalue UBLOCK_PHAGS_PA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PHAGS_PA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PHOENICIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PHOENICIAN = UNKNOWN;
    /** @cvalue UBLOCK_PHOENICIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PHOENICIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CUNEIFORM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CUNEIFORM = UNKNOWN;
    /** @cvalue UBLOCK_CUNEIFORM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CUNEIFORM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CUNEIFORM_NUMBERS_AND_PUNCTUATION = UNKNOWN;
    /** @cvalue UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CUNEIFORM_NUMBERS_AND_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COUNTING_ROD_NUMERALS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COUNTING_ROD_NUMERALS = UNKNOWN;
    /** @cvalue UBLOCK_COUNTING_ROD_NUMERALS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COUNTING_ROD_NUMERALS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUNDANESE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUNDANESE = UNKNOWN;
    /** @cvalue UBLOCK_SUNDANESE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUNDANESE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LEPCHA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LEPCHA = UNKNOWN;
    /** @cvalue UBLOCK_LEPCHA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LEPCHA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OL_CHIKI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OL_CHIKI = UNKNOWN;
    /** @cvalue UBLOCK_OL_CHIKI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OL_CHIKI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CYRILLIC_EXTENDED_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CYRILLIC_EXTENDED_A = UNKNOWN;
    /** @cvalue UBLOCK_CYRILLIC_EXTENDED_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CYRILLIC_EXTENDED_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_VAI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_VAI = UNKNOWN;
    /** @cvalue UBLOCK_VAI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_VAI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CYRILLIC_EXTENDED_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CYRILLIC_EXTENDED_B = UNKNOWN;
    /** @cvalue UBLOCK_CYRILLIC_EXTENDED_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CYRILLIC_EXTENDED_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SAURASHTRA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SAURASHTRA = UNKNOWN;
    /** @cvalue UBLOCK_SAURASHTRA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SAURASHTRA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KAYAH_LI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KAYAH_LI = UNKNOWN;
    /** @cvalue UBLOCK_KAYAH_LI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KAYAH_LI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_REJANG
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_REJANG = UNKNOWN;
    /** @cvalue UBLOCK_REJANG */
    #[\Since('8.4')]
    public const int BLOCK_CODE_REJANG = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CHAM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CHAM = UNKNOWN;
    /** @cvalue UBLOCK_CHAM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CHAM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ANCIENT_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ANCIENT_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_ANCIENT_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ANCIENT_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PHAISTOS_DISC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PHAISTOS_DISC = UNKNOWN;
    /** @cvalue UBLOCK_PHAISTOS_DISC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PHAISTOS_DISC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LYCIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LYCIAN = UNKNOWN;
    /** @cvalue UBLOCK_LYCIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LYCIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CARIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CARIAN = UNKNOWN;
    /** @cvalue UBLOCK_CARIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CARIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LYDIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LYDIAN = UNKNOWN;
    /** @cvalue UBLOCK_LYDIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LYDIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MAHJONG_TILES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MAHJONG_TILES = UNKNOWN;
    /** @cvalue UBLOCK_MAHJONG_TILES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MAHJONG_TILES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_DOMINO_TILES
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_DOMINO_TILES = UNKNOWN;
    /** @cvalue UBLOCK_DOMINO_TILES */
    #[\Since('8.4')]
    public const int BLOCK_CODE_DOMINO_TILES = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SAMARITAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SAMARITAN = UNKNOWN;
    /** @cvalue UBLOCK_SAMARITAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SAMARITAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAI_THAM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAI_THAM = UNKNOWN;
    /** @cvalue UBLOCK_TAI_THAM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAI_THAM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_VEDIC_EXTENSIONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_VEDIC_EXTENSIONS = UNKNOWN;
    /** @cvalue UBLOCK_VEDIC_EXTENSIONS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_VEDIC_EXTENSIONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LISU
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LISU = UNKNOWN;
    /** @cvalue UBLOCK_LISU */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LISU = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BAMUM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BAMUM = UNKNOWN;
    /** @cvalue UBLOCK_BAMUM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BAMUM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COMMON_INDIC_NUMBER_FORMS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COMMON_INDIC_NUMBER_FORMS = UNKNOWN;
    /** @cvalue UBLOCK_COMMON_INDIC_NUMBER_FORMS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COMMON_INDIC_NUMBER_FORMS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_DEVANAGARI_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_DEVANAGARI_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_DEVANAGARI_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_DEVANAGARI_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HANGUL_JAMO_EXTENDED_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HANGUL_JAMO_EXTENDED_A = UNKNOWN;
    /** @cvalue UBLOCK_HANGUL_JAMO_EXTENDED_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HANGUL_JAMO_EXTENDED_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_JAVANESE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_JAVANESE = UNKNOWN;
    /** @cvalue UBLOCK_JAVANESE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_JAVANESE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MYANMAR_EXTENDED_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MYANMAR_EXTENDED_A = UNKNOWN;
    /** @cvalue UBLOCK_MYANMAR_EXTENDED_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MYANMAR_EXTENDED_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAI_VIET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAI_VIET = UNKNOWN;
    /** @cvalue UBLOCK_TAI_VIET */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAI_VIET = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MEETEI_MAYEK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MEETEI_MAYEK = UNKNOWN;
    /** @cvalue UBLOCK_MEETEI_MAYEK */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MEETEI_MAYEK = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_HANGUL_JAMO_EXTENDED_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_HANGUL_JAMO_EXTENDED_B = UNKNOWN;
    /** @cvalue UBLOCK_HANGUL_JAMO_EXTENDED_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_HANGUL_JAMO_EXTENDED_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_IMPERIAL_ARAMAIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_IMPERIAL_ARAMAIC = UNKNOWN;
    /** @cvalue UBLOCK_IMPERIAL_ARAMAIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_IMPERIAL_ARAMAIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OLD_SOUTH_ARABIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OLD_SOUTH_ARABIAN = UNKNOWN;
    /** @cvalue UBLOCK_OLD_SOUTH_ARABIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OLD_SOUTH_ARABIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_AVESTAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_AVESTAN = UNKNOWN;
    /** @cvalue UBLOCK_AVESTAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_AVESTAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_INSCRIPTIONAL_PARTHIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_INSCRIPTIONAL_PARTHIAN = UNKNOWN;
    /** @cvalue UBLOCK_INSCRIPTIONAL_PARTHIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_INSCRIPTIONAL_PARTHIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_INSCRIPTIONAL_PAHLAVI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_INSCRIPTIONAL_PAHLAVI = UNKNOWN;
    /** @cvalue UBLOCK_INSCRIPTIONAL_PAHLAVI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_INSCRIPTIONAL_PAHLAVI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OLD_TURKIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OLD_TURKIC = UNKNOWN;
    /** @cvalue UBLOCK_OLD_TURKIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OLD_TURKIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_RUMI_NUMERAL_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_RUMI_NUMERAL_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_RUMI_NUMERAL_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_RUMI_NUMERAL_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KAITHI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KAITHI = UNKNOWN;
    /** @cvalue UBLOCK_KAITHI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KAITHI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_EGYPTIAN_HIEROGLYPHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_EGYPTIAN_HIEROGLYPHS = UNKNOWN;
    /** @cvalue UBLOCK_EGYPTIAN_HIEROGLYPHS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_EGYPTIAN_HIEROGLYPHS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ENCLOSED_ALPHANUMERIC_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ENCLOSED_ALPHANUMERIC_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C = UNKNOWN;
    /** @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MANDAIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MANDAIC = UNKNOWN;
    /** @cvalue UBLOCK_MANDAIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MANDAIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BATAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BATAK = UNKNOWN;
    /** @cvalue UBLOCK_BATAK */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BATAK = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ETHIOPIC_EXTENDED_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ETHIOPIC_EXTENDED_A = UNKNOWN;
    /** @cvalue UBLOCK_ETHIOPIC_EXTENDED_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ETHIOPIC_EXTENDED_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BRAHMI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BRAHMI = UNKNOWN;
    /** @cvalue UBLOCK_BRAHMI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BRAHMI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_BAMUM_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BAMUM_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_BAMUM_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BAMUM_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KANA_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KANA_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_KANA_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KANA_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PLAYING_CARDS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PLAYING_CARDS = UNKNOWN;
    /** @cvalue UBLOCK_PLAYING_CARDS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PLAYING_CARDS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS = UNKNOWN;
    /** @cvalue UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_EMOTICONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_EMOTICONS = UNKNOWN;
    /** @cvalue UBLOCK_EMOTICONS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_EMOTICONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TRANSPORT_AND_MAP_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TRANSPORT_AND_MAP_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_TRANSPORT_AND_MAP_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TRANSPORT_AND_MAP_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ALCHEMICAL_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ALCHEMICAL_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_ALCHEMICAL_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ALCHEMICAL_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D = UNKNOWN;
    /** @cvalue UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARABIC_EXTENDED_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARABIC_EXTENDED_A = UNKNOWN;
    /** @cvalue UBLOCK_ARABIC_EXTENDED_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARABIC_EXTENDED_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS = UNKNOWN;
    /** @cvalue UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CHAKMA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CHAKMA = UNKNOWN;
    /** @cvalue UBLOCK_CHAKMA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CHAKMA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MEETEI_MAYEK_EXTENSIONS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MEETEI_MAYEK_EXTENSIONS = UNKNOWN;
    /** @cvalue UBLOCK_MEETEI_MAYEK_EXTENSIONS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MEETEI_MAYEK_EXTENSIONS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MEROITIC_CURSIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MEROITIC_CURSIVE = UNKNOWN;
    /** @cvalue UBLOCK_MEROITIC_CURSIVE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MEROITIC_CURSIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MEROITIC_HIEROGLYPHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MEROITIC_HIEROGLYPHS = UNKNOWN;
    /** @cvalue UBLOCK_MEROITIC_HIEROGLYPHS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MEROITIC_HIEROGLYPHS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MIAO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MIAO = UNKNOWN;
    /** @cvalue UBLOCK_MIAO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MIAO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SHARADA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SHARADA = UNKNOWN;
    /** @cvalue UBLOCK_SHARADA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SHARADA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SORA_SOMPENG
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SORA_SOMPENG = UNKNOWN;
    /** @cvalue UBLOCK_SORA_SOMPENG */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SORA_SOMPENG = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUNDANESE_SUPPLEMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUNDANESE_SUPPLEMENT = UNKNOWN;
    /** @cvalue UBLOCK_SUNDANESE_SUPPLEMENT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUNDANESE_SUPPLEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TAKRI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TAKRI = UNKNOWN;
    /** @cvalue UBLOCK_TAKRI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TAKRI = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 54
    /**
     * @var int
     * @cvalue UBLOCK_BASSA_VAH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_BASSA_VAH = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 54
    /** @cvalue UBLOCK_BASSA_VAH */
    #[\Since('8.4')]
    public const int BLOCK_CODE_BASSA_VAH = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_CAUCASIAN_ALBANIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_CAUCASIAN_ALBANIAN = UNKNOWN;
    /** @cvalue UBLOCK_CAUCASIAN_ALBANIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_CAUCASIAN_ALBANIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COPTIC_EPACT_NUMBERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COPTIC_EPACT_NUMBERS = UNKNOWN;
    /** @cvalue UBLOCK_COPTIC_EPACT_NUMBERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COPTIC_EPACT_NUMBERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COMBINING_DIACRITICAL_MARKS_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COMBINING_DIACRITICAL_MARKS_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_DUPLOYAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_DUPLOYAN = UNKNOWN;
    /** @cvalue UBLOCK_DUPLOYAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_DUPLOYAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ELBASAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ELBASAN = UNKNOWN;
    /** @cvalue UBLOCK_ELBASAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ELBASAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GEOMETRIC_SHAPES_EXTENDED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GEOMETRIC_SHAPES_EXTENDED = UNKNOWN;
    /** @cvalue UBLOCK_GEOMETRIC_SHAPES_EXTENDED */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GEOMETRIC_SHAPES_EXTENDED = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_GRANTHA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_GRANTHA = UNKNOWN;
    /** @cvalue UBLOCK_GRANTHA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_GRANTHA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KHOJKI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KHOJKI = UNKNOWN;
    /** @cvalue UBLOCK_KHOJKI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KHOJKI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_KHUDAWADI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_KHUDAWADI = UNKNOWN;
    /** @cvalue UBLOCK_KHUDAWADI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_KHUDAWADI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LATIN_EXTENDED_E
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LATIN_EXTENDED_E = UNKNOWN;
    /** @cvalue UBLOCK_LATIN_EXTENDED_E */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LATIN_EXTENDED_E = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_LINEAR_A
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_LINEAR_A = UNKNOWN;
    /** @cvalue UBLOCK_LINEAR_A */
    #[\Since('8.4')]
    public const int BLOCK_CODE_LINEAR_A = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MAHAJANI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MAHAJANI = UNKNOWN;
    /** @cvalue UBLOCK_MAHAJANI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MAHAJANI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MANICHAEAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MANICHAEAN = UNKNOWN;
    /** @cvalue UBLOCK_MANICHAEAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MANICHAEAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MENDE_KIKAKUI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MENDE_KIKAKUI = UNKNOWN;
    /** @cvalue UBLOCK_MENDE_KIKAKUI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MENDE_KIKAKUI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MODI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MODI = UNKNOWN;
    /** @cvalue UBLOCK_MODI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MODI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MRO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MRO = UNKNOWN;
    /** @cvalue UBLOCK_MRO */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MRO = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_MYANMAR_EXTENDED_B
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_MYANMAR_EXTENDED_B = UNKNOWN;
    /** @cvalue UBLOCK_MYANMAR_EXTENDED_B */
    #[\Since('8.4')]
    public const int BLOCK_CODE_MYANMAR_EXTENDED_B = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_NABATAEAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_NABATAEAN = UNKNOWN;
    /** @cvalue UBLOCK_NABATAEAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_NABATAEAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OLD_NORTH_ARABIAN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OLD_NORTH_ARABIAN = UNKNOWN;
    /** @cvalue UBLOCK_OLD_NORTH_ARABIAN */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OLD_NORTH_ARABIAN = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_OLD_PERMIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_OLD_PERMIC = UNKNOWN;
    /** @cvalue UBLOCK_OLD_PERMIC */
    #[\Since('8.4')]
    public const int BLOCK_CODE_OLD_PERMIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_ORNAMENTAL_DINGBATS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_ORNAMENTAL_DINGBATS = UNKNOWN;
    /** @cvalue UBLOCK_ORNAMENTAL_DINGBATS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_ORNAMENTAL_DINGBATS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PAHAWH_HMONG
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PAHAWH_HMONG = UNKNOWN;
    /** @cvalue UBLOCK_PAHAWH_HMONG */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PAHAWH_HMONG = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PALMYRENE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PALMYRENE = UNKNOWN;
    /** @cvalue UBLOCK_PALMYRENE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PALMYRENE = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PAU_CIN_HAU
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PAU_CIN_HAU = UNKNOWN;
    /** @cvalue UBLOCK_PAU_CIN_HAU */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PAU_CIN_HAU = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_PSALTER_PAHLAVI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_PSALTER_PAHLAVI = UNKNOWN;
    /** @cvalue UBLOCK_PSALTER_PAHLAVI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_PSALTER_PAHLAVI = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SHORTHAND_FORMAT_CONTROLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SHORTHAND_FORMAT_CONTROLS = UNKNOWN;
    /** @cvalue UBLOCK_SHORTHAND_FORMAT_CONTROLS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SHORTHAND_FORMAT_CONTROLS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SIDDHAM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SIDDHAM = UNKNOWN;
    /** @cvalue UBLOCK_SIDDHAM */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SIDDHAM = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SINHALA_ARCHAIC_NUMBERS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SINHALA_ARCHAIC_NUMBERS = UNKNOWN;
    /** @cvalue UBLOCK_SINHALA_ARCHAIC_NUMBERS */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SINHALA_ARCHAIC_NUMBERS = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_SUPPLEMENTAL_ARROWS_C
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_SUPPLEMENTAL_ARROWS_C = UNKNOWN;
    /** @cvalue UBLOCK_SUPPLEMENTAL_ARROWS_C */
    #[\Since('8.4')]
    public const int BLOCK_CODE_SUPPLEMENTAL_ARROWS_C = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_TIRHUTA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_TIRHUTA = UNKNOWN;
    /** @cvalue UBLOCK_TIRHUTA */
    #[\Since('8.4')]
    public const int BLOCK_CODE_TIRHUTA = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_WARANG_CITI
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_WARANG_CITI = UNKNOWN;
    /** @cvalue UBLOCK_WARANG_CITI */
    #[\Since('8.4')]
    public const int BLOCK_CODE_WARANG_CITI = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue UBLOCK_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_COUNT = UNKNOWN;
    #endif
    /** @cvalue UBLOCK_COUNT */
    #[\Since('8.4')]
    public const int BLOCK_CODE_COUNT = UNKNOWN;
    /**
     * @var int
     * @cvalue UBLOCK_INVALID_CODE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BLOCK_CODE_INVALID_CODE = UNKNOWN;
    /** @cvalue UBLOCK_INVALID_CODE */
    #[\Since('8.4')]
    public const int BLOCK_CODE_INVALID_CODE = UNKNOWN;
    /* UBidiPairedBracketType - http://icu-project.org/apiref/icu4c/uchar_8h.html#af954219aa1df452657ec355221c6703d */
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @var int
     * @cvalue U_BPT_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BPT_NONE = UNKNOWN;
    /* UBidiPairedBracketType - http://icu-project.org/apiref/icu4c/uchar_8h.html#af954219aa1df452657ec355221c6703d */
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /** @cvalue U_BPT_NONE */
    #[\Since('8.4')]
    public const int BPT_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_BPT_OPEN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BPT_OPEN = UNKNOWN;
    /** @cvalue U_BPT_OPEN */
    #[\Since('8.4')]
    public const int BPT_OPEN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_BPT_CLOSE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BPT_CLOSE = UNKNOWN;
    /** @cvalue U_BPT_CLOSE */
    #[\Since('8.4')]
    public const int BPT_CLOSE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_BPT_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BPT_COUNT = UNKNOWN;
    /** @cvalue U_BPT_COUNT */
    #[\Since('8.4')]
    public const int BPT_COUNT = UNKNOWN;
    #endif
    /* UEastAsianWidth - http://icu-project.org/apiref/icu4c/uchar_8h.html#a95cc2ca2f9cfd6d0c63eee2c65951333 */
    /**
     * @var int
     * @cvalue U_EA_NEUTRAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_NEUTRAL = UNKNOWN;
    #endif
    /* UEastAsianWidth - http://icu-project.org/apiref/icu4c/uchar_8h.html#a95cc2ca2f9cfd6d0c63eee2c65951333 */
    /** @cvalue U_EA_NEUTRAL */
    #[\Since('8.4')]
    public const int EA_NEUTRAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EA_AMBIGUOUS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_AMBIGUOUS = UNKNOWN;
    /** @cvalue U_EA_AMBIGUOUS */
    #[\Since('8.4')]
    public const int EA_AMBIGUOUS = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EA_HALFWIDTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_HALFWIDTH = UNKNOWN;
    /** @cvalue U_EA_HALFWIDTH */
    #[\Since('8.4')]
    public const int EA_HALFWIDTH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EA_FULLWIDTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_FULLWIDTH = UNKNOWN;
    /** @cvalue U_EA_FULLWIDTH */
    #[\Since('8.4')]
    public const int EA_FULLWIDTH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EA_NARROW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_NARROW = UNKNOWN;
    /** @cvalue U_EA_NARROW */
    #[\Since('8.4')]
    public const int EA_NARROW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EA_WIDE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_WIDE = UNKNOWN;
    /** @cvalue U_EA_WIDE */
    #[\Since('8.4')]
    public const int EA_WIDE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EA_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EA_COUNT = UNKNOWN;
    /** @cvalue U_EA_COUNT */
    #[\Since('8.4')]
    public const int EA_COUNT = UNKNOWN;
    /* UCharNameChoice - http://icu-project.org/apiref/icu4c/uchar_8h.html#a2ba37edcca62eff48226e8096035addf */
    /**
     * @var int
     * @cvalue U_UNICODE_CHAR_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UNICODE_CHAR_NAME = UNKNOWN;
    /* UCharNameChoice - http://icu-project.org/apiref/icu4c/uchar_8h.html#a2ba37edcca62eff48226e8096035addf */
    /** @cvalue U_UNICODE_CHAR_NAME */
    #[\Since('8.4')]
    public const int UNICODE_CHAR_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue U_UNICODE_10_CHAR_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UNICODE_10_CHAR_NAME = UNKNOWN;
    /** @cvalue U_UNICODE_10_CHAR_NAME */
    #[\Since('8.4')]
    public const int UNICODE_10_CHAR_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue U_EXTENDED_CHAR_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EXTENDED_CHAR_NAME = UNKNOWN;
    /** @cvalue U_EXTENDED_CHAR_NAME */
    #[\Since('8.4')]
    public const int EXTENDED_CHAR_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue U_CHAR_NAME_ALIAS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_NAME_ALIAS = UNKNOWN;
    /** @cvalue U_CHAR_NAME_ALIAS */
    #[\Since('8.4')]
    public const int CHAR_NAME_ALIAS = UNKNOWN;
    /**
     * @var int
     * @cvalue U_CHAR_NAME_CHOICE_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_NAME_CHOICE_COUNT = UNKNOWN;
    /** @cvalue U_CHAR_NAME_CHOICE_COUNT */
    #[\Since('8.4')]
    public const int CHAR_NAME_CHOICE_COUNT = UNKNOWN;
    /* UPropertyNameChoice - http://icu-project.org/apiref/icu4c/uchar_8h.html#a5056494c7d5a2c7185f3c464f48fe5d1 */
    /**
     * @var int
     * @cvalue U_SHORT_PROPERTY_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SHORT_PROPERTY_NAME = UNKNOWN;
    /* UPropertyNameChoice - http://icu-project.org/apiref/icu4c/uchar_8h.html#a5056494c7d5a2c7185f3c464f48fe5d1 */
    /** @cvalue U_SHORT_PROPERTY_NAME */
    #[\Since('8.4')]
    public const int SHORT_PROPERTY_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LONG_PROPERTY_NAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LONG_PROPERTY_NAME = UNKNOWN;
    /** @cvalue U_LONG_PROPERTY_NAME */
    #[\Since('8.4')]
    public const int LONG_PROPERTY_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue U_PROPERTY_NAME_CHOICE_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PROPERTY_NAME_CHOICE_COUNT = UNKNOWN;
    /** @cvalue U_PROPERTY_NAME_CHOICE_COUNT */
    #[\Since('8.4')]
    public const int PROPERTY_NAME_CHOICE_COUNT = UNKNOWN;
    /* UDecompositionType - http://icu-project.org/apiref/icu4c/uchar_8h.html#ae2c56994fcf28062c7e77beb671533f5 */
    /**
     * @var int
     * @cvalue U_DT_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_NONE = UNKNOWN;
    /* UDecompositionType - http://icu-project.org/apiref/icu4c/uchar_8h.html#ae2c56994fcf28062c7e77beb671533f5 */
    /** @cvalue U_DT_NONE */
    #[\Since('8.4')]
    public const int DT_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_CANONICAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_CANONICAL = UNKNOWN;
    /** @cvalue U_DT_CANONICAL */
    #[\Since('8.4')]
    public const int DT_CANONICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_COMPAT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_COMPAT = UNKNOWN;
    /** @cvalue U_DT_COMPAT */
    #[\Since('8.4')]
    public const int DT_COMPAT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_CIRCLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_CIRCLE = UNKNOWN;
    /** @cvalue U_DT_CIRCLE */
    #[\Since('8.4')]
    public const int DT_CIRCLE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_FINAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_FINAL = UNKNOWN;
    /** @cvalue U_DT_FINAL */
    #[\Since('8.4')]
    public const int DT_FINAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_FONT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_FONT = UNKNOWN;
    /** @cvalue U_DT_FONT */
    #[\Since('8.4')]
    public const int DT_FONT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_FRACTION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_FRACTION = UNKNOWN;
    /** @cvalue U_DT_FRACTION */
    #[\Since('8.4')]
    public const int DT_FRACTION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_INITIAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_INITIAL = UNKNOWN;
    /** @cvalue U_DT_INITIAL */
    #[\Since('8.4')]
    public const int DT_INITIAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_ISOLATED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_ISOLATED = UNKNOWN;
    /** @cvalue U_DT_ISOLATED */
    #[\Since('8.4')]
    public const int DT_ISOLATED = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_MEDIAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_MEDIAL = UNKNOWN;
    /** @cvalue U_DT_MEDIAL */
    #[\Since('8.4')]
    public const int DT_MEDIAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_NARROW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_NARROW = UNKNOWN;
    /** @cvalue U_DT_NARROW */
    #[\Since('8.4')]
    public const int DT_NARROW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_NOBREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_NOBREAK = UNKNOWN;
    /** @cvalue U_DT_NOBREAK */
    #[\Since('8.4')]
    public const int DT_NOBREAK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_SMALL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_SMALL = UNKNOWN;
    /** @cvalue U_DT_SMALL */
    #[\Since('8.4')]
    public const int DT_SMALL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_SQUARE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_SQUARE = UNKNOWN;
    /** @cvalue U_DT_SQUARE */
    #[\Since('8.4')]
    public const int DT_SQUARE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_SUB
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_SUB = UNKNOWN;
    /** @cvalue U_DT_SUB */
    #[\Since('8.4')]
    public const int DT_SUB = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_SUPER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_SUPER = UNKNOWN;
    /** @cvalue U_DT_SUPER */
    #[\Since('8.4')]
    public const int DT_SUPER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_VERTICAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_VERTICAL = UNKNOWN;
    /** @cvalue U_DT_VERTICAL */
    #[\Since('8.4')]
    public const int DT_VERTICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_WIDE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_WIDE = UNKNOWN;
    /** @cvalue U_DT_WIDE */
    #[\Since('8.4')]
    public const int DT_WIDE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_DT_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DT_COUNT = UNKNOWN;
    /** @cvalue U_DT_COUNT */
    #[\Since('8.4')]
    public const int DT_COUNT = UNKNOWN;
    /* UJoiningType - http://icu-project.org/apiref/icu4c/uchar_8h.html#a3ce1ce20e7f3b8534eb3490ad3aba3dd */
    /**
     * @var int
     * @cvalue U_JT_NON_JOINING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_NON_JOINING = UNKNOWN;
    /* UJoiningType - http://icu-project.org/apiref/icu4c/uchar_8h.html#a3ce1ce20e7f3b8534eb3490ad3aba3dd */
    /** @cvalue U_JT_NON_JOINING */
    #[\Since('8.4')]
    public const int JT_NON_JOINING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JT_JOIN_CAUSING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_JOIN_CAUSING = UNKNOWN;
    /** @cvalue U_JT_JOIN_CAUSING */
    #[\Since('8.4')]
    public const int JT_JOIN_CAUSING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JT_DUAL_JOINING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_DUAL_JOINING = UNKNOWN;
    /** @cvalue U_JT_DUAL_JOINING */
    #[\Since('8.4')]
    public const int JT_DUAL_JOINING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JT_LEFT_JOINING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_LEFT_JOINING = UNKNOWN;
    /** @cvalue U_JT_LEFT_JOINING */
    #[\Since('8.4')]
    public const int JT_LEFT_JOINING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JT_RIGHT_JOINING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_RIGHT_JOINING = UNKNOWN;
    /** @cvalue U_JT_RIGHT_JOINING */
    #[\Since('8.4')]
    public const int JT_RIGHT_JOINING = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JT_TRANSPARENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_TRANSPARENT = UNKNOWN;
    /** @cvalue U_JT_TRANSPARENT */
    #[\Since('8.4')]
    public const int JT_TRANSPARENT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JT_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JT_COUNT = UNKNOWN;
    /** @cvalue U_JT_COUNT */
    #[\Since('8.4')]
    public const int JT_COUNT = UNKNOWN;
    /* UJoiningGroup - http://icu-project.org/apiref/icu4c/uchar_8h.html#a7887844ec0872e6e9a283e0825fcae65 */
    /**
     * @var int
     * @cvalue U_JG_NO_JOINING_GROUP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_NO_JOINING_GROUP = UNKNOWN;
    /* UJoiningGroup - http://icu-project.org/apiref/icu4c/uchar_8h.html#a7887844ec0872e6e9a283e0825fcae65 */
    /** @cvalue U_JG_NO_JOINING_GROUP */
    #[\Since('8.4')]
    public const int JG_NO_JOINING_GROUP = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_AIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_AIN = UNKNOWN;
    /** @cvalue U_JG_AIN */
    #[\Since('8.4')]
    public const int JG_AIN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_ALAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_ALAPH = UNKNOWN;
    /** @cvalue U_JG_ALAPH */
    #[\Since('8.4')]
    public const int JG_ALAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_ALEF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_ALEF = UNKNOWN;
    /** @cvalue U_JG_ALEF */
    #[\Since('8.4')]
    public const int JG_ALEF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_BEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_BEH = UNKNOWN;
    /** @cvalue U_JG_BEH */
    #[\Since('8.4')]
    public const int JG_BEH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_BETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_BETH = UNKNOWN;
    /** @cvalue U_JG_BETH */
    #[\Since('8.4')]
    public const int JG_BETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_DAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_DAL = UNKNOWN;
    /** @cvalue U_JG_DAL */
    #[\Since('8.4')]
    public const int JG_DAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_DALATH_RISH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_DALATH_RISH = UNKNOWN;
    /** @cvalue U_JG_DALATH_RISH */
    #[\Since('8.4')]
    public const int JG_DALATH_RISH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_E
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_E = UNKNOWN;
    /** @cvalue U_JG_E */
    #[\Since('8.4')]
    public const int JG_E = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_FEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_FEH = UNKNOWN;
    /** @cvalue U_JG_FEH */
    #[\Since('8.4')]
    public const int JG_FEH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_FINAL_SEMKATH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_FINAL_SEMKATH = UNKNOWN;
    /** @cvalue U_JG_FINAL_SEMKATH */
    #[\Since('8.4')]
    public const int JG_FINAL_SEMKATH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_GAF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_GAF = UNKNOWN;
    /** @cvalue U_JG_GAF */
    #[\Since('8.4')]
    public const int JG_GAF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_GAMAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_GAMAL = UNKNOWN;
    /** @cvalue U_JG_GAMAL */
    #[\Since('8.4')]
    public const int JG_GAMAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_HAH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_HAH = UNKNOWN;
    /** @cvalue U_JG_HAH */
    #[\Since('8.4')]
    public const int JG_HAH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_TEH_MARBUTA_GOAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_TEH_MARBUTA_GOAL = UNKNOWN;
    /** @cvalue U_JG_TEH_MARBUTA_GOAL */
    #[\Since('8.4')]
    public const int JG_TEH_MARBUTA_GOAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_HAMZA_ON_HEH_GOAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_HAMZA_ON_HEH_GOAL = UNKNOWN;
    /** @cvalue U_JG_HAMZA_ON_HEH_GOAL */
    #[\Since('8.4')]
    public const int JG_HAMZA_ON_HEH_GOAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_HE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_HE = UNKNOWN;
    /** @cvalue U_JG_HE */
    #[\Since('8.4')]
    public const int JG_HE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_HEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_HEH = UNKNOWN;
    /** @cvalue U_JG_HEH */
    #[\Since('8.4')]
    public const int JG_HEH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_HEH_GOAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_HEH_GOAL = UNKNOWN;
    /** @cvalue U_JG_HEH_GOAL */
    #[\Since('8.4')]
    public const int JG_HEH_GOAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_HETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_HETH = UNKNOWN;
    /** @cvalue U_JG_HETH */
    #[\Since('8.4')]
    public const int JG_HETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_KAF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_KAF = UNKNOWN;
    /** @cvalue U_JG_KAF */
    #[\Since('8.4')]
    public const int JG_KAF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_KAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_KAPH = UNKNOWN;
    /** @cvalue U_JG_KAPH */
    #[\Since('8.4')]
    public const int JG_KAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_KNOTTED_HEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_KNOTTED_HEH = UNKNOWN;
    /** @cvalue U_JG_KNOTTED_HEH */
    #[\Since('8.4')]
    public const int JG_KNOTTED_HEH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_LAM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_LAM = UNKNOWN;
    /** @cvalue U_JG_LAM */
    #[\Since('8.4')]
    public const int JG_LAM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_LAMADH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_LAMADH = UNKNOWN;
    /** @cvalue U_JG_LAMADH */
    #[\Since('8.4')]
    public const int JG_LAMADH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MEEM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MEEM = UNKNOWN;
    /** @cvalue U_JG_MEEM */
    #[\Since('8.4')]
    public const int JG_MEEM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MIM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MIM = UNKNOWN;
    /** @cvalue U_JG_MIM */
    #[\Since('8.4')]
    public const int JG_MIM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_NOON
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_NOON = UNKNOWN;
    /** @cvalue U_JG_NOON */
    #[\Since('8.4')]
    public const int JG_NOON = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_NUN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_NUN = UNKNOWN;
    /** @cvalue U_JG_NUN */
    #[\Since('8.4')]
    public const int JG_NUN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_PE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_PE = UNKNOWN;
    /** @cvalue U_JG_PE */
    #[\Since('8.4')]
    public const int JG_PE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_QAF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_QAF = UNKNOWN;
    /** @cvalue U_JG_QAF */
    #[\Since('8.4')]
    public const int JG_QAF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_QAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_QAPH = UNKNOWN;
    /** @cvalue U_JG_QAPH */
    #[\Since('8.4')]
    public const int JG_QAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_REH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_REH = UNKNOWN;
    /** @cvalue U_JG_REH */
    #[\Since('8.4')]
    public const int JG_REH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_REVERSED_PE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_REVERSED_PE = UNKNOWN;
    /** @cvalue U_JG_REVERSED_PE */
    #[\Since('8.4')]
    public const int JG_REVERSED_PE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SAD
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SAD = UNKNOWN;
    /** @cvalue U_JG_SAD */
    #[\Since('8.4')]
    public const int JG_SAD = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SADHE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SADHE = UNKNOWN;
    /** @cvalue U_JG_SADHE */
    #[\Since('8.4')]
    public const int JG_SADHE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SEEN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SEEN = UNKNOWN;
    /** @cvalue U_JG_SEEN */
    #[\Since('8.4')]
    public const int JG_SEEN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SEMKATH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SEMKATH = UNKNOWN;
    /** @cvalue U_JG_SEMKATH */
    #[\Since('8.4')]
    public const int JG_SEMKATH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SHIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SHIN = UNKNOWN;
    /** @cvalue U_JG_SHIN */
    #[\Since('8.4')]
    public const int JG_SHIN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SWASH_KAF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SWASH_KAF = UNKNOWN;
    /** @cvalue U_JG_SWASH_KAF */
    #[\Since('8.4')]
    public const int JG_SWASH_KAF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_SYRIAC_WAW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_SYRIAC_WAW = UNKNOWN;
    /** @cvalue U_JG_SYRIAC_WAW */
    #[\Since('8.4')]
    public const int JG_SYRIAC_WAW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_TAH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_TAH = UNKNOWN;
    /** @cvalue U_JG_TAH */
    #[\Since('8.4')]
    public const int JG_TAH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_TAW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_TAW = UNKNOWN;
    /** @cvalue U_JG_TAW */
    #[\Since('8.4')]
    public const int JG_TAW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_TEH_MARBUTA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_TEH_MARBUTA = UNKNOWN;
    /** @cvalue U_JG_TEH_MARBUTA */
    #[\Since('8.4')]
    public const int JG_TEH_MARBUTA = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_TETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_TETH = UNKNOWN;
    /** @cvalue U_JG_TETH */
    #[\Since('8.4')]
    public const int JG_TETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_WAW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_WAW = UNKNOWN;
    /** @cvalue U_JG_WAW */
    #[\Since('8.4')]
    public const int JG_WAW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_YEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_YEH = UNKNOWN;
    /** @cvalue U_JG_YEH */
    #[\Since('8.4')]
    public const int JG_YEH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_YEH_BARREE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_YEH_BARREE = UNKNOWN;
    /** @cvalue U_JG_YEH_BARREE */
    #[\Since('8.4')]
    public const int JG_YEH_BARREE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_YEH_WITH_TAIL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_YEH_WITH_TAIL = UNKNOWN;
    /** @cvalue U_JG_YEH_WITH_TAIL */
    #[\Since('8.4')]
    public const int JG_YEH_WITH_TAIL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_YUDH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_YUDH = UNKNOWN;
    /** @cvalue U_JG_YUDH */
    #[\Since('8.4')]
    public const int JG_YUDH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_YUDH_HE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_YUDH_HE = UNKNOWN;
    /** @cvalue U_JG_YUDH_HE */
    #[\Since('8.4')]
    public const int JG_YUDH_HE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_ZAIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_ZAIN = UNKNOWN;
    /** @cvalue U_JG_ZAIN */
    #[\Since('8.4')]
    public const int JG_ZAIN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_FE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_FE = UNKNOWN;
    /** @cvalue U_JG_FE */
    #[\Since('8.4')]
    public const int JG_FE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_KHAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_KHAPH = UNKNOWN;
    /** @cvalue U_JG_KHAPH */
    #[\Since('8.4')]
    public const int JG_KHAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_ZHAIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_ZHAIN = UNKNOWN;
    /** @cvalue U_JG_ZHAIN */
    #[\Since('8.4')]
    public const int JG_ZHAIN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_BURUSHASKI_YEH_BARREE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_BURUSHASKI_YEH_BARREE = UNKNOWN;
    /** @cvalue U_JG_BURUSHASKI_YEH_BARREE */
    #[\Since('8.4')]
    public const int JG_BURUSHASKI_YEH_BARREE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_FARSI_YEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_FARSI_YEH = UNKNOWN;
    /** @cvalue U_JG_FARSI_YEH */
    #[\Since('8.4')]
    public const int JG_FARSI_YEH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_NYA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_NYA = UNKNOWN;
    /** @cvalue U_JG_NYA */
    #[\Since('8.4')]
    public const int JG_NYA = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_ROHINGYA_YEH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_ROHINGYA_YEH = UNKNOWN;
    /** @cvalue U_JG_ROHINGYA_YEH */
    #[\Since('8.4')]
    public const int JG_ROHINGYA_YEH = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 54
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_ALEPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_ALEPH = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 54
    /** @cvalue U_JG_MANICHAEAN_ALEPH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_ALEPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_AYIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_AYIN = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_AYIN */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_AYIN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_BETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_BETH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_BETH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_BETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_DALETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_DALETH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_DALETH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_DALETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_DHAMEDH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_DHAMEDH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_DHAMEDH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_DHAMEDH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_FIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_FIVE = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_FIVE */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_FIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_GIMEL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_GIMEL = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_GIMEL */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_GIMEL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_HETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_HETH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_HETH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_HETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_HUNDRED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_HUNDRED = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_HUNDRED */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_HUNDRED = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_KAPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_KAPH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_KAPH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_KAPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_LAMEDH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_LAMEDH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_LAMEDH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_LAMEDH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_MEM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_MEM = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_MEM */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_MEM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_NUN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_NUN = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_NUN */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_NUN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_ONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_ONE = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_ONE */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_ONE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_PE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_PE = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_PE */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_PE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_QOPH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_QOPH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_QOPH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_QOPH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_RESH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_RESH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_RESH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_RESH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_SADHE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_SADHE = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_SADHE */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_SADHE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_SAMEKH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_SAMEKH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_SAMEKH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_SAMEKH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_TAW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_TAW = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_TAW */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_TAW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_TEN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_TEN = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_TEN */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_TEN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_TETH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_TETH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_TETH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_TETH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_THAMEDH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_THAMEDH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_THAMEDH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_THAMEDH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_TWENTY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_TWENTY = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_TWENTY */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_TWENTY = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_WAW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_WAW = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_WAW */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_WAW = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_YODH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_YODH = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_YODH */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_YODH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_MANICHAEAN_ZAYIN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_MANICHAEAN_ZAYIN = UNKNOWN;
    /** @cvalue U_JG_MANICHAEAN_ZAYIN */
    #[\Since('8.4')]
    public const int JG_MANICHAEAN_ZAYIN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_JG_STRAIGHT_WAW
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_STRAIGHT_WAW = UNKNOWN;
    /** @cvalue U_JG_STRAIGHT_WAW */
    #[\Since('8.4')]
    public const int JG_STRAIGHT_WAW = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue U_JG_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const JG_COUNT = UNKNOWN;
    #endif
    /** @cvalue U_JG_COUNT */
    #[\Since('8.4')]
    public const int JG_COUNT = UNKNOWN;
    /* UGraphemeClusterBreak - http://icu-project.org/apiref/icu4c/uchar_8h.html#abb9bae7d2a1c80ce342be4647661fde1 */
    /**
     * @var int
     * @cvalue U_GCB_OTHER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_OTHER = UNKNOWN;
    /* UGraphemeClusterBreak - http://icu-project.org/apiref/icu4c/uchar_8h.html#abb9bae7d2a1c80ce342be4647661fde1 */
    /** @cvalue U_GCB_OTHER */
    #[\Since('8.4')]
    public const int GCB_OTHER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_CONTROL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_CONTROL = UNKNOWN;
    /** @cvalue U_GCB_CONTROL */
    #[\Since('8.4')]
    public const int GCB_CONTROL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_CR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_CR = UNKNOWN;
    /** @cvalue U_GCB_CR */
    #[\Since('8.4')]
    public const int GCB_CR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_EXTEND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_EXTEND = UNKNOWN;
    /** @cvalue U_GCB_EXTEND */
    #[\Since('8.4')]
    public const int GCB_EXTEND = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_L
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_L = UNKNOWN;
    /** @cvalue U_GCB_L */
    #[\Since('8.4')]
    public const int GCB_L = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_LF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_LF = UNKNOWN;
    /** @cvalue U_GCB_LF */
    #[\Since('8.4')]
    public const int GCB_LF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_LV
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_LV = UNKNOWN;
    /** @cvalue U_GCB_LV */
    #[\Since('8.4')]
    public const int GCB_LV = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_LVT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_LVT = UNKNOWN;
    /** @cvalue U_GCB_LVT */
    #[\Since('8.4')]
    public const int GCB_LVT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_T
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_T = UNKNOWN;
    /** @cvalue U_GCB_T */
    #[\Since('8.4')]
    public const int GCB_T = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_V
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_V = UNKNOWN;
    /** @cvalue U_GCB_V */
    #[\Since('8.4')]
    public const int GCB_V = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_SPACING_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_SPACING_MARK = UNKNOWN;
    /** @cvalue U_GCB_SPACING_MARK */
    #[\Since('8.4')]
    public const int GCB_SPACING_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_PREPEND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_PREPEND = UNKNOWN;
    /** @cvalue U_GCB_PREPEND */
    #[\Since('8.4')]
    public const int GCB_PREPEND = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_REGIONAL_INDICATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_REGIONAL_INDICATOR = UNKNOWN;
    /** @cvalue U_GCB_REGIONAL_INDICATOR */
    #[\Since('8.4')]
    public const int GCB_REGIONAL_INDICATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_GCB_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GCB_COUNT = UNKNOWN;
    /** @cvalue U_GCB_COUNT */
    #[\Since('8.4')]
    public const int GCB_COUNT = UNKNOWN;
    /* UWordBreakValues - http://icu-project.org/apiref/icu4c/uchar_8h.html#af70ee907368e663f8dd4b90c7196e15c */
    /**
     * @var int
     * @cvalue U_WB_OTHER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_OTHER = UNKNOWN;
    /* UWordBreakValues - http://icu-project.org/apiref/icu4c/uchar_8h.html#af70ee907368e663f8dd4b90c7196e15c */
    /** @cvalue U_WB_OTHER */
    #[\Since('8.4')]
    public const int WB_OTHER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_ALETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_ALETTER = UNKNOWN;
    /** @cvalue U_WB_ALETTER */
    #[\Since('8.4')]
    public const int WB_ALETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_FORMAT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_FORMAT = UNKNOWN;
    /** @cvalue U_WB_FORMAT */
    #[\Since('8.4')]
    public const int WB_FORMAT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_KATAKANA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_KATAKANA = UNKNOWN;
    /** @cvalue U_WB_KATAKANA */
    #[\Since('8.4')]
    public const int WB_KATAKANA = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_MIDLETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_MIDLETTER = UNKNOWN;
    /** @cvalue U_WB_MIDLETTER */
    #[\Since('8.4')]
    public const int WB_MIDLETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_MIDNUM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_MIDNUM = UNKNOWN;
    /** @cvalue U_WB_MIDNUM */
    #[\Since('8.4')]
    public const int WB_MIDNUM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_NUMERIC = UNKNOWN;
    /** @cvalue U_WB_NUMERIC */
    #[\Since('8.4')]
    public const int WB_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_EXTENDNUMLET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_EXTENDNUMLET = UNKNOWN;
    /** @cvalue U_WB_EXTENDNUMLET */
    #[\Since('8.4')]
    public const int WB_EXTENDNUMLET = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_CR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_CR = UNKNOWN;
    /** @cvalue U_WB_CR */
    #[\Since('8.4')]
    public const int WB_CR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_EXTEND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_EXTEND = UNKNOWN;
    /** @cvalue U_WB_EXTEND */
    #[\Since('8.4')]
    public const int WB_EXTEND = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_LF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_LF = UNKNOWN;
    /** @cvalue U_WB_LF */
    #[\Since('8.4')]
    public const int WB_LF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_MIDNUMLET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_MIDNUMLET = UNKNOWN;
    /** @cvalue U_WB_MIDNUMLET */
    #[\Since('8.4')]
    public const int WB_MIDNUMLET = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_NEWLINE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_NEWLINE = UNKNOWN;
    /** @cvalue U_WB_NEWLINE */
    #[\Since('8.4')]
    public const int WB_NEWLINE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_REGIONAL_INDICATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_REGIONAL_INDICATOR = UNKNOWN;
    /** @cvalue U_WB_REGIONAL_INDICATOR */
    #[\Since('8.4')]
    public const int WB_REGIONAL_INDICATOR = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @var int
     * @cvalue U_WB_HEBREW_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_HEBREW_LETTER = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /** @cvalue U_WB_HEBREW_LETTER */
    #[\Since('8.4')]
    public const int WB_HEBREW_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_SINGLE_QUOTE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_SINGLE_QUOTE = UNKNOWN;
    /** @cvalue U_WB_SINGLE_QUOTE */
    #[\Since('8.4')]
    public const int WB_SINGLE_QUOTE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_WB_DOUBLE_QUOTE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_DOUBLE_QUOTE = UNKNOWN;
    /** @cvalue U_WB_DOUBLE_QUOTE */
    #[\Since('8.4')]
    public const int WB_DOUBLE_QUOTE = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue U_WB_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WB_COUNT = UNKNOWN;
    #endif
    /** @cvalue U_WB_COUNT */
    #[\Since('8.4')]
    public const int WB_COUNT = UNKNOWN;
    /* USentenceBreak - http://icu-project.org/apiref/icu4c/uchar_8h.html#a89e9e463c3bae1d2d46b1dbb6f90de0f */
    /**
     * @var int
     * @cvalue U_SB_OTHER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_OTHER = UNKNOWN;
    /* USentenceBreak - http://icu-project.org/apiref/icu4c/uchar_8h.html#a89e9e463c3bae1d2d46b1dbb6f90de0f */
    /** @cvalue U_SB_OTHER */
    #[\Since('8.4')]
    public const int SB_OTHER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_ATERM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_ATERM = UNKNOWN;
    /** @cvalue U_SB_ATERM */
    #[\Since('8.4')]
    public const int SB_ATERM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_CLOSE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_CLOSE = UNKNOWN;
    /** @cvalue U_SB_CLOSE */
    #[\Since('8.4')]
    public const int SB_CLOSE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_FORMAT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_FORMAT = UNKNOWN;
    /** @cvalue U_SB_FORMAT */
    #[\Since('8.4')]
    public const int SB_FORMAT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_LOWER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_LOWER = UNKNOWN;
    /** @cvalue U_SB_LOWER */
    #[\Since('8.4')]
    public const int SB_LOWER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_NUMERIC = UNKNOWN;
    /** @cvalue U_SB_NUMERIC */
    #[\Since('8.4')]
    public const int SB_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_OLETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_OLETTER = UNKNOWN;
    /** @cvalue U_SB_OLETTER */
    #[\Since('8.4')]
    public const int SB_OLETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_SEP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_SEP = UNKNOWN;
    /** @cvalue U_SB_SEP */
    #[\Since('8.4')]
    public const int SB_SEP = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_SP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_SP = UNKNOWN;
    /** @cvalue U_SB_SP */
    #[\Since('8.4')]
    public const int SB_SP = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_STERM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_STERM = UNKNOWN;
    /** @cvalue U_SB_STERM */
    #[\Since('8.4')]
    public const int SB_STERM = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_UPPER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_UPPER = UNKNOWN;
    /** @cvalue U_SB_UPPER */
    #[\Since('8.4')]
    public const int SB_UPPER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_CR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_CR = UNKNOWN;
    /** @cvalue U_SB_CR */
    #[\Since('8.4')]
    public const int SB_CR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_EXTEND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_EXTEND = UNKNOWN;
    /** @cvalue U_SB_EXTEND */
    #[\Since('8.4')]
    public const int SB_EXTEND = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_LF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_LF = UNKNOWN;
    /** @cvalue U_SB_LF */
    #[\Since('8.4')]
    public const int SB_LF = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_SCONTINUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_SCONTINUE = UNKNOWN;
    /** @cvalue U_SB_SCONTINUE */
    #[\Since('8.4')]
    public const int SB_SCONTINUE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_SB_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SB_COUNT = UNKNOWN;
    /** @cvalue U_SB_COUNT */
    #[\Since('8.4')]
    public const int SB_COUNT = UNKNOWN;
    /* ULineBreak - http://icu-project.org/apiref/icu4c/uchar_8h.html#a5d1abdf05be22cb9599f804a8506277c */
    /**
     * @var int
     * @cvalue U_LB_UNKNOWN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_UNKNOWN = UNKNOWN;
    /* ULineBreak - http://icu-project.org/apiref/icu4c/uchar_8h.html#a5d1abdf05be22cb9599f804a8506277c */
    /** @cvalue U_LB_UNKNOWN */
    #[\Since('8.4')]
    public const int LB_UNKNOWN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_AMBIGUOUS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_AMBIGUOUS = UNKNOWN;
    /** @cvalue U_LB_AMBIGUOUS */
    #[\Since('8.4')]
    public const int LB_AMBIGUOUS = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_ALPHABETIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_ALPHABETIC = UNKNOWN;
    /** @cvalue U_LB_ALPHABETIC */
    #[\Since('8.4')]
    public const int LB_ALPHABETIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_BREAK_BOTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_BREAK_BOTH = UNKNOWN;
    /** @cvalue U_LB_BREAK_BOTH */
    #[\Since('8.4')]
    public const int LB_BREAK_BOTH = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_BREAK_AFTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_BREAK_AFTER = UNKNOWN;
    /** @cvalue U_LB_BREAK_AFTER */
    #[\Since('8.4')]
    public const int LB_BREAK_AFTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_BREAK_BEFORE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_BREAK_BEFORE = UNKNOWN;
    /** @cvalue U_LB_BREAK_BEFORE */
    #[\Since('8.4')]
    public const int LB_BREAK_BEFORE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_MANDATORY_BREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_MANDATORY_BREAK = UNKNOWN;
    /** @cvalue U_LB_MANDATORY_BREAK */
    #[\Since('8.4')]
    public const int LB_MANDATORY_BREAK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_CONTINGENT_BREAK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_CONTINGENT_BREAK = UNKNOWN;
    /** @cvalue U_LB_CONTINGENT_BREAK */
    #[\Since('8.4')]
    public const int LB_CONTINGENT_BREAK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_CLOSE_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_CLOSE_PUNCTUATION = UNKNOWN;
    /** @cvalue U_LB_CLOSE_PUNCTUATION */
    #[\Since('8.4')]
    public const int LB_CLOSE_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_COMBINING_MARK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_COMBINING_MARK = UNKNOWN;
    /** @cvalue U_LB_COMBINING_MARK */
    #[\Since('8.4')]
    public const int LB_COMBINING_MARK = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_CARRIAGE_RETURN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_CARRIAGE_RETURN = UNKNOWN;
    /** @cvalue U_LB_CARRIAGE_RETURN */
    #[\Since('8.4')]
    public const int LB_CARRIAGE_RETURN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_EXCLAMATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_EXCLAMATION = UNKNOWN;
    /** @cvalue U_LB_EXCLAMATION */
    #[\Since('8.4')]
    public const int LB_EXCLAMATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_GLUE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_GLUE = UNKNOWN;
    /** @cvalue U_LB_GLUE */
    #[\Since('8.4')]
    public const int LB_GLUE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_HYPHEN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_HYPHEN = UNKNOWN;
    /** @cvalue U_LB_HYPHEN */
    #[\Since('8.4')]
    public const int LB_HYPHEN = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_IDEOGRAPHIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_IDEOGRAPHIC = UNKNOWN;
    /** @cvalue U_LB_IDEOGRAPHIC */
    #[\Since('8.4')]
    public const int LB_IDEOGRAPHIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_INSEPARABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_INSEPARABLE = UNKNOWN;
    /** @cvalue U_LB_INSEPARABLE */
    #[\Since('8.4')]
    public const int LB_INSEPARABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_INSEPERABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_INSEPERABLE = UNKNOWN;
    /** @cvalue U_LB_INSEPERABLE */
    #[\Since('8.4')]
    public const int LB_INSEPERABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_INFIX_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_INFIX_NUMERIC = UNKNOWN;
    /** @cvalue U_LB_INFIX_NUMERIC */
    #[\Since('8.4')]
    public const int LB_INFIX_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_LINE_FEED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_LINE_FEED = UNKNOWN;
    /** @cvalue U_LB_LINE_FEED */
    #[\Since('8.4')]
    public const int LB_LINE_FEED = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_NONSTARTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_NONSTARTER = UNKNOWN;
    /** @cvalue U_LB_NONSTARTER */
    #[\Since('8.4')]
    public const int LB_NONSTARTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_NUMERIC = UNKNOWN;
    /** @cvalue U_LB_NUMERIC */
    #[\Since('8.4')]
    public const int LB_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_OPEN_PUNCTUATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_OPEN_PUNCTUATION = UNKNOWN;
    /** @cvalue U_LB_OPEN_PUNCTUATION */
    #[\Since('8.4')]
    public const int LB_OPEN_PUNCTUATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_POSTFIX_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_POSTFIX_NUMERIC = UNKNOWN;
    /** @cvalue U_LB_POSTFIX_NUMERIC */
    #[\Since('8.4')]
    public const int LB_POSTFIX_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_PREFIX_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_PREFIX_NUMERIC = UNKNOWN;
    /** @cvalue U_LB_PREFIX_NUMERIC */
    #[\Since('8.4')]
    public const int LB_PREFIX_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_QUOTATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_QUOTATION = UNKNOWN;
    /** @cvalue U_LB_QUOTATION */
    #[\Since('8.4')]
    public const int LB_QUOTATION = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_COMPLEX_CONTEXT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_COMPLEX_CONTEXT = UNKNOWN;
    /** @cvalue U_LB_COMPLEX_CONTEXT */
    #[\Since('8.4')]
    public const int LB_COMPLEX_CONTEXT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_SURROGATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_SURROGATE = UNKNOWN;
    /** @cvalue U_LB_SURROGATE */
    #[\Since('8.4')]
    public const int LB_SURROGATE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_SPACE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_SPACE = UNKNOWN;
    /** @cvalue U_LB_SPACE */
    #[\Since('8.4')]
    public const int LB_SPACE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_BREAK_SYMBOLS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_BREAK_SYMBOLS = UNKNOWN;
    /** @cvalue U_LB_BREAK_SYMBOLS */
    #[\Since('8.4')]
    public const int LB_BREAK_SYMBOLS = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_ZWSPACE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_ZWSPACE = UNKNOWN;
    /** @cvalue U_LB_ZWSPACE */
    #[\Since('8.4')]
    public const int LB_ZWSPACE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_NEXT_LINE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_NEXT_LINE = UNKNOWN;
    /** @cvalue U_LB_NEXT_LINE */
    #[\Since('8.4')]
    public const int LB_NEXT_LINE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_WORD_JOINER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_WORD_JOINER = UNKNOWN;
    /** @cvalue U_LB_WORD_JOINER */
    #[\Since('8.4')]
    public const int LB_WORD_JOINER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_H2
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_H2 = UNKNOWN;
    /** @cvalue U_LB_H2 */
    #[\Since('8.4')]
    public const int LB_H2 = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_H3
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_H3 = UNKNOWN;
    /** @cvalue U_LB_H3 */
    #[\Since('8.4')]
    public const int LB_H3 = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_JL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_JL = UNKNOWN;
    /** @cvalue U_LB_JL */
    #[\Since('8.4')]
    public const int LB_JL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_JT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_JT = UNKNOWN;
    /** @cvalue U_LB_JT */
    #[\Since('8.4')]
    public const int LB_JT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_JV
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_JV = UNKNOWN;
    /** @cvalue U_LB_JV */
    #[\Since('8.4')]
    public const int LB_JV = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_CLOSE_PARENTHESIS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_CLOSE_PARENTHESIS = UNKNOWN;
    /** @cvalue U_LB_CLOSE_PARENTHESIS */
    #[\Since('8.4')]
    public const int LB_CLOSE_PARENTHESIS = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_CONDITIONAL_JAPANESE_STARTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_CONDITIONAL_JAPANESE_STARTER = UNKNOWN;
    /** @cvalue U_LB_CONDITIONAL_JAPANESE_STARTER */
    #[\Since('8.4')]
    public const int LB_CONDITIONAL_JAPANESE_STARTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_HEBREW_LETTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_HEBREW_LETTER = UNKNOWN;
    /** @cvalue U_LB_HEBREW_LETTER */
    #[\Since('8.4')]
    public const int LB_HEBREW_LETTER = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_REGIONAL_INDICATOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_REGIONAL_INDICATOR = UNKNOWN;
    /** @cvalue U_LB_REGIONAL_INDICATOR */
    #[\Since('8.4')]
    public const int LB_REGIONAL_INDICATOR = UNKNOWN;
    /**
     * @var int
     * @cvalue U_LB_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LB_COUNT = UNKNOWN;
    /** @cvalue U_LB_COUNT */
    #[\Since('8.4')]
    public const int LB_COUNT = UNKNOWN;
    /* UNumericType - http://icu-project.org/apiref/icu4c/uchar_8h.html#adec3e7a6ae3a00274c019b3b2ddaecbe */
    /**
     * @var int
     * @cvalue U_NT_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NT_NONE = UNKNOWN;
    /* UNumericType - http://icu-project.org/apiref/icu4c/uchar_8h.html#adec3e7a6ae3a00274c019b3b2ddaecbe */
    /** @cvalue U_NT_NONE */
    #[\Since('8.4')]
    public const int NT_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_NT_DECIMAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NT_DECIMAL = UNKNOWN;
    /** @cvalue U_NT_DECIMAL */
    #[\Since('8.4')]
    public const int NT_DECIMAL = UNKNOWN;
    /**
     * @var int
     * @cvalue U_NT_DIGIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NT_DIGIT = UNKNOWN;
    /** @cvalue U_NT_DIGIT */
    #[\Since('8.4')]
    public const int NT_DIGIT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_NT_NUMERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NT_NUMERIC = UNKNOWN;
    /** @cvalue U_NT_NUMERIC */
    #[\Since('8.4')]
    public const int NT_NUMERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue U_NT_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NT_COUNT = UNKNOWN;
    /** @cvalue U_NT_COUNT */
    #[\Since('8.4')]
    public const int NT_COUNT = UNKNOWN;
    /* UHangulSyllableType - http://icu-project.org/apiref/icu4c/uchar_8h.html#a7cb09027c37ad73571cf541caf002c8f */
    /**
     * @var int
     * @cvalue U_HST_NOT_APPLICABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_NOT_APPLICABLE = UNKNOWN;
    /* UHangulSyllableType - http://icu-project.org/apiref/icu4c/uchar_8h.html#a7cb09027c37ad73571cf541caf002c8f */
    /** @cvalue U_HST_NOT_APPLICABLE */
    #[\Since('8.4')]
    public const int HST_NOT_APPLICABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_HST_LEADING_JAMO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_LEADING_JAMO = UNKNOWN;
    /** @cvalue U_HST_LEADING_JAMO */
    #[\Since('8.4')]
    public const int HST_LEADING_JAMO = UNKNOWN;
    /**
     * @var int
     * @cvalue U_HST_VOWEL_JAMO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_VOWEL_JAMO = UNKNOWN;
    /** @cvalue U_HST_VOWEL_JAMO */
    #[\Since('8.4')]
    public const int HST_VOWEL_JAMO = UNKNOWN;
    /**
     * @var int
     * @cvalue U_HST_TRAILING_JAMO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_TRAILING_JAMO = UNKNOWN;
    /** @cvalue U_HST_TRAILING_JAMO */
    #[\Since('8.4')]
    public const int HST_TRAILING_JAMO = UNKNOWN;
    /**
     * @var int
     * @cvalue U_HST_LV_SYLLABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_LV_SYLLABLE = UNKNOWN;
    /** @cvalue U_HST_LV_SYLLABLE */
    #[\Since('8.4')]
    public const int HST_LV_SYLLABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_HST_LVT_SYLLABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_LVT_SYLLABLE = UNKNOWN;
    /** @cvalue U_HST_LVT_SYLLABLE */
    #[\Since('8.4')]
    public const int HST_LVT_SYLLABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue U_HST_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HST_COUNT = UNKNOWN;
    /** @cvalue U_HST_COUNT */
    #[\Since('8.4')]
    public const int HST_COUNT = UNKNOWN;
    /* StringOptions - http://icu-project.org/apiref/icu4c/stringoptions_8h.html */
    /**
     * @var int
     * @cvalue U_FOLD_CASE_DEFAULT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FOLD_CASE_DEFAULT = UNKNOWN;
    /* StringOptions - http://icu-project.org/apiref/icu4c/stringoptions_8h.html */
    /** @cvalue U_FOLD_CASE_DEFAULT */
    #[\Since('8.4')]
    public const int FOLD_CASE_DEFAULT = UNKNOWN;
    /**
     * @var int
     * @cvalue U_FOLD_CASE_EXCLUDE_SPECIAL_I
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FOLD_CASE_EXCLUDE_SPECIAL_I = UNKNOWN;
    /** @cvalue U_FOLD_CASE_EXCLUDE_SPECIAL_I */
    #[\Since('8.4')]
    public const int FOLD_CASE_EXCLUDE_SPECIAL_I = UNKNOWN;
}