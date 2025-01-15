<?php 

/** @generate-function-entries */
class NumberFormatter
{
    /**
     * @var int
     * @cvalue FORMAT_TYPE_CURRENCY
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_CURRENCY = UNKNOWN;
    public function __construct(string $locale, int $style, ?string $pattern = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_create
     * @return (NumberFormatter | null)
     */
    public static function create(string $locale, int $style, ?string $pattern = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_format
     * @return (string | false)
     */
    public function format(int|float $num, int $type = NumberFormatter::TYPE_DEFAULT)
    {
    }
    /**
     * @param int $offset
     * @tentative-return-type
     * @alias numfmt_parse
     * @return (int | float | false)
     */
    public function parse(string $string, int $type = NumberFormatter::TYPE_DOUBLE, &$offset = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_format_currency
     * @return (string | false)
     */
    public function formatCurrency(float $amount, string $currency)
    {
    }
    /**
     * @param string $currency
     * @param int $offset
     * @tentative-return-type
     * @alias numfmt_parse_currency
     * @return (float | false)
     */
    public function parseCurrency(string $string, &$currency, &$offset = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_set_attribute
     * @return bool
     */
    public function setAttribute(int $attribute, int|float $value)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_attribute
     * @return (int | float | false)
     */
    public function getAttribute(int $attribute)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_set_text_attribute
     * @return bool
     */
    public function setTextAttribute(int $attribute, string $value)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_text_attribute
     * @return (string | false)
     */
    public function getTextAttribute(int $attribute)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_set_symbol
     * @return bool
     */
    public function setSymbol(int $symbol, string $value)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_symbol
     * @return (string | false)
     */
    public function getSymbol(int $symbol)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_set_pattern
     * @return bool
     */
    public function setPattern(string $pattern)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_pattern
     * @return (string | false)
     */
    public function getPattern()
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_locale
     * @return (string | false)
     */
    public function getLocale(int $type = ULOC_ACTUAL_LOCALE)
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_error_code
     * @return int
     */
    public function getErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @alias numfmt_get_error_message
     * @return string
     */
    public function getErrorMessage()
    {
    }
    /* UNumberFormatStyle constants */
    /**
     * @var int
     * @cvalue UNUM_PATTERN_DECIMAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PATTERN_DECIMAL = UNKNOWN;
    /* UNumberFormatStyle constants */
    /** @cvalue UNUM_PATTERN_DECIMAL */
    #[\Since('8.4')]
    public const int PATTERN_DECIMAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_DECIMAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DECIMAL = UNKNOWN;
    /** @cvalue UNUM_DECIMAL */
    #[\Since('8.4')]
    public const int DECIMAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_CURRENCY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENCY = UNKNOWN;
    /** @cvalue UNUM_CURRENCY */
    #[\Since('8.4')]
    public const int CURRENCY = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PERCENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PERCENT = UNKNOWN;
    /** @cvalue UNUM_PERCENT */
    #[\Since('8.4')]
    public const int PERCENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_SCIENTIFIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SCIENTIFIC = UNKNOWN;
    /** @cvalue UNUM_SCIENTIFIC */
    #[\Since('8.4')]
    public const int SCIENTIFIC = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_SPELLOUT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SPELLOUT = UNKNOWN;
    /** @cvalue UNUM_SPELLOUT */
    #[\Since('8.4')]
    public const int SPELLOUT = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ORDINAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ORDINAL = UNKNOWN;
    /** @cvalue UNUM_ORDINAL */
    #[\Since('8.4')]
    public const int ORDINAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_DURATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DURATION = UNKNOWN;
    /** @cvalue UNUM_DURATION */
    #[\Since('8.4')]
    public const int DURATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PATTERN_RULEBASED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PATTERN_RULEBASED = UNKNOWN;
    /** @cvalue UNUM_PATTERN_RULEBASED */
    #[\Since('8.4')]
    public const int PATTERN_RULEBASED = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_IGNORE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IGNORE = UNKNOWN;
    /** @cvalue UNUM_IGNORE */
    #[\Since('8.4')]
    public const int IGNORE = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 53
    /**
     * @var int
     * @cvalue UNUM_CURRENCY_ACCOUNTING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENCY_ACCOUNTING = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 53
    /** @cvalue UNUM_CURRENCY_ACCOUNTING */
    #[\Since('8.4')]
    public const int CURRENCY_ACCOUNTING = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue UNUM_DEFAULT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DEFAULT_STYLE = UNKNOWN;
    #endif
    /** @cvalue UNUM_DEFAULT */
    #[\Since('8.4')]
    public const int DEFAULT_STYLE = UNKNOWN;
    /* UNumberFormatRoundingMode */
    /**
     * @var int
     * @cvalue UNUM_ROUND_CEILING
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_CEILING = UNKNOWN;
    /* UNumberFormatRoundingMode */
    /** @cvalue UNUM_ROUND_CEILING */
    #[\Since('8.4')]
    public const int ROUND_CEILING = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUND_FLOOR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_FLOOR = UNKNOWN;
    /** @cvalue UNUM_ROUND_FLOOR */
    #[\Since('8.4')]
    public const int ROUND_FLOOR = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUND_DOWN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_DOWN = UNKNOWN;
    /** @cvalue UNUM_ROUND_DOWN */
    #[\Since('8.4')]
    public const int ROUND_DOWN = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUND_UP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_UP = UNKNOWN;
    /** @cvalue UNUM_ROUND_UP */
    #[\Since('8.4')]
    public const int ROUND_UP = UNKNOWN;
    /** @cvalue UNUM_ROUND_DOWN */
    #[\Since('8.4')]
    public const int ROUND_TOWARD_ZERO = UNKNOWN;
    /** @cvalue UNUM_ROUND_UP */
    #[\Since('8.4')]
    public const int ROUND_AWAY_FROM_ZERO = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUND_HALFEVEN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_HALFEVEN = UNKNOWN;
    /** @cvalue UNUM_ROUND_HALFEVEN */
    #[\Since('8.4')]
    public const int ROUND_HALFEVEN = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 69
    /** @cvalue UNUM_ROUND_HALF_ODD */
    #[\Since('8.4')]
    public const int ROUND_HALFODD = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUND_HALFDOWN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_HALFDOWN = UNKNOWN;
    #endif
    /** @cvalue UNUM_ROUND_HALFDOWN */
    #[\Since('8.4')]
    public const int ROUND_HALFDOWN = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUND_HALFUP
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUND_HALFUP = UNKNOWN;
    /** @cvalue UNUM_ROUND_HALFUP */
    #[\Since('8.4')]
    public const int ROUND_HALFUP = UNKNOWN;
    /* UNumberFormatPadPosition */
    /**
     * @var int
     * @cvalue UNUM_PAD_BEFORE_PREFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PAD_BEFORE_PREFIX = UNKNOWN;
    /* UNumberFormatPadPosition */
    /** @cvalue UNUM_PAD_BEFORE_PREFIX */
    #[\Since('8.4')]
    public const int PAD_BEFORE_PREFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PAD_AFTER_PREFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PAD_AFTER_PREFIX = UNKNOWN;
    /** @cvalue UNUM_PAD_AFTER_PREFIX */
    #[\Since('8.4')]
    public const int PAD_AFTER_PREFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PAD_BEFORE_SUFFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PAD_BEFORE_SUFFIX = UNKNOWN;
    /** @cvalue UNUM_PAD_BEFORE_SUFFIX */
    #[\Since('8.4')]
    public const int PAD_BEFORE_SUFFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PAD_AFTER_SUFFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PAD_AFTER_SUFFIX = UNKNOWN;
    /** @cvalue UNUM_PAD_AFTER_SUFFIX */
    #[\Since('8.4')]
    public const int PAD_AFTER_SUFFIX = UNKNOWN;
    /* UNumberFormatAttribute */
    /**
     * @var int
     * @cvalue UNUM_PARSE_INT_ONLY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARSE_INT_ONLY = UNKNOWN;
    /* UNumberFormatAttribute */
    /** @cvalue UNUM_PARSE_INT_ONLY */
    #[\Since('8.4')]
    public const int PARSE_INT_ONLY = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_GROUPING_USED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GROUPING_USED = UNKNOWN;
    /** @cvalue UNUM_GROUPING_USED */
    #[\Since('8.4')]
    public const int GROUPING_USED = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_DECIMAL_ALWAYS_SHOWN
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DECIMAL_ALWAYS_SHOWN = UNKNOWN;
    /** @cvalue UNUM_DECIMAL_ALWAYS_SHOWN */
    #[\Since('8.4')]
    public const int DECIMAL_ALWAYS_SHOWN = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MAX_INTEGER_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MAX_INTEGER_DIGITS = UNKNOWN;
    /** @cvalue UNUM_MAX_INTEGER_DIGITS */
    #[\Since('8.4')]
    public const int MAX_INTEGER_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MIN_INTEGER_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MIN_INTEGER_DIGITS = UNKNOWN;
    /** @cvalue UNUM_MIN_INTEGER_DIGITS */
    #[\Since('8.4')]
    public const int MIN_INTEGER_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_INTEGER_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const INTEGER_DIGITS = UNKNOWN;
    /** @cvalue UNUM_INTEGER_DIGITS */
    #[\Since('8.4')]
    public const int INTEGER_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MAX_FRACTION_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MAX_FRACTION_DIGITS = UNKNOWN;
    /** @cvalue UNUM_MAX_FRACTION_DIGITS */
    #[\Since('8.4')]
    public const int MAX_FRACTION_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MIN_FRACTION_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MIN_FRACTION_DIGITS = UNKNOWN;
    /** @cvalue UNUM_MIN_FRACTION_DIGITS */
    #[\Since('8.4')]
    public const int MIN_FRACTION_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_FRACTION_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FRACTION_DIGITS = UNKNOWN;
    /** @cvalue UNUM_FRACTION_DIGITS */
    #[\Since('8.4')]
    public const int FRACTION_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MULTIPLIER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MULTIPLIER = UNKNOWN;
    /** @cvalue UNUM_MULTIPLIER */
    #[\Since('8.4')]
    public const int MULTIPLIER = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_GROUPING_SIZE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GROUPING_SIZE = UNKNOWN;
    /** @cvalue UNUM_GROUPING_SIZE */
    #[\Since('8.4')]
    public const int GROUPING_SIZE = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUNDING_MODE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUNDING_MODE = UNKNOWN;
    /** @cvalue UNUM_ROUNDING_MODE */
    #[\Since('8.4')]
    public const int ROUNDING_MODE = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ROUNDING_INCREMENT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ROUNDING_INCREMENT = UNKNOWN;
    /** @cvalue UNUM_ROUNDING_INCREMENT */
    #[\Since('8.4')]
    public const int ROUNDING_INCREMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_FORMAT_WIDTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FORMAT_WIDTH = UNKNOWN;
    /** @cvalue UNUM_FORMAT_WIDTH */
    #[\Since('8.4')]
    public const int FORMAT_WIDTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PADDING_POSITION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PADDING_POSITION = UNKNOWN;
    /** @cvalue UNUM_PADDING_POSITION */
    #[\Since('8.4')]
    public const int PADDING_POSITION = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_SECONDARY_GROUPING_SIZE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SECONDARY_GROUPING_SIZE = UNKNOWN;
    /** @cvalue UNUM_SECONDARY_GROUPING_SIZE */
    #[\Since('8.4')]
    public const int SECONDARY_GROUPING_SIZE = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_SIGNIFICANT_DIGITS_USED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SIGNIFICANT_DIGITS_USED = UNKNOWN;
    /** @cvalue UNUM_SIGNIFICANT_DIGITS_USED */
    #[\Since('8.4')]
    public const int SIGNIFICANT_DIGITS_USED = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MIN_SIGNIFICANT_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MIN_SIGNIFICANT_DIGITS = UNKNOWN;
    /** @cvalue UNUM_MIN_SIGNIFICANT_DIGITS */
    #[\Since('8.4')]
    public const int MIN_SIGNIFICANT_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MAX_SIGNIFICANT_DIGITS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MAX_SIGNIFICANT_DIGITS = UNKNOWN;
    /** @cvalue UNUM_MAX_SIGNIFICANT_DIGITS */
    #[\Since('8.4')]
    public const int MAX_SIGNIFICANT_DIGITS = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_LENIENT_PARSE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LENIENT_PARSE = UNKNOWN;
    /** @cvalue UNUM_LENIENT_PARSE */
    #[\Since('8.4')]
    public const int LENIENT_PARSE = UNKNOWN;
    /* UNumberFormatTextAttribute */
    /**
     * @var int
     * @cvalue UNUM_POSITIVE_PREFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const POSITIVE_PREFIX = UNKNOWN;
    /* UNumberFormatTextAttribute */
    /** @cvalue UNUM_POSITIVE_PREFIX */
    #[\Since('8.4')]
    public const int POSITIVE_PREFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_POSITIVE_SUFFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const POSITIVE_SUFFIX = UNKNOWN;
    /** @cvalue UNUM_POSITIVE_SUFFIX */
    #[\Since('8.4')]
    public const int POSITIVE_SUFFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_NEGATIVE_PREFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NEGATIVE_PREFIX = UNKNOWN;
    /** @cvalue UNUM_NEGATIVE_PREFIX */
    #[\Since('8.4')]
    public const int NEGATIVE_PREFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_NEGATIVE_SUFFIX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NEGATIVE_SUFFIX = UNKNOWN;
    /** @cvalue UNUM_NEGATIVE_SUFFIX */
    #[\Since('8.4')]
    public const int NEGATIVE_SUFFIX = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PADDING_CHARACTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PADDING_CHARACTER = UNKNOWN;
    /** @cvalue UNUM_PADDING_CHARACTER */
    #[\Since('8.4')]
    public const int PADDING_CHARACTER = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_CURRENCY_CODE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENCY_CODE = UNKNOWN;
    /** @cvalue UNUM_CURRENCY_CODE */
    #[\Since('8.4')]
    public const int CURRENCY_CODE = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_DEFAULT_RULESET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DEFAULT_RULESET = UNKNOWN;
    /** @cvalue UNUM_DEFAULT_RULESET */
    #[\Since('8.4')]
    public const int DEFAULT_RULESET = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PUBLIC_RULESETS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PUBLIC_RULESETS = UNKNOWN;
    /** @cvalue UNUM_PUBLIC_RULESETS */
    #[\Since('8.4')]
    public const int PUBLIC_RULESETS = UNKNOWN;
    /* UNumberFormatSymbol */
    /**
     * @var int
     * @cvalue UNUM_DECIMAL_SEPARATOR_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DECIMAL_SEPARATOR_SYMBOL = UNKNOWN;
    /* UNumberFormatSymbol */
    /** @cvalue UNUM_DECIMAL_SEPARATOR_SYMBOL */
    #[\Since('8.4')]
    public const int DECIMAL_SEPARATOR_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_GROUPING_SEPARATOR_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const GROUPING_SEPARATOR_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_GROUPING_SEPARATOR_SYMBOL */
    #[\Since('8.4')]
    public const int GROUPING_SEPARATOR_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PATTERN_SEPARATOR_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PATTERN_SEPARATOR_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_PATTERN_SEPARATOR_SYMBOL */
    #[\Since('8.4')]
    public const int PATTERN_SEPARATOR_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PERCENT_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PERCENT_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_PERCENT_SYMBOL */
    #[\Since('8.4')]
    public const int PERCENT_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_ZERO_DIGIT_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ZERO_DIGIT_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_ZERO_DIGIT_SYMBOL */
    #[\Since('8.4')]
    public const int ZERO_DIGIT_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_DIGIT_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DIGIT_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_DIGIT_SYMBOL */
    #[\Since('8.4')]
    public const int DIGIT_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MINUS_SIGN_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MINUS_SIGN_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_MINUS_SIGN_SYMBOL */
    #[\Since('8.4')]
    public const int MINUS_SIGN_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PLUS_SIGN_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PLUS_SIGN_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_PLUS_SIGN_SYMBOL */
    #[\Since('8.4')]
    public const int PLUS_SIGN_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_CURRENCY_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENCY_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_CURRENCY_SYMBOL */
    #[\Since('8.4')]
    public const int CURRENCY_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_INTL_CURRENCY_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const INTL_CURRENCY_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_INTL_CURRENCY_SYMBOL */
    #[\Since('8.4')]
    public const int INTL_CURRENCY_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MONETARY_SEPARATOR_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MONETARY_SEPARATOR_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_MONETARY_SEPARATOR_SYMBOL */
    #[\Since('8.4')]
    public const int MONETARY_SEPARATOR_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_EXPONENTIAL_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EXPONENTIAL_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_EXPONENTIAL_SYMBOL */
    #[\Since('8.4')]
    public const int EXPONENTIAL_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PERMILL_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PERMILL_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_PERMILL_SYMBOL */
    #[\Since('8.4')]
    public const int PERMILL_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_PAD_ESCAPE_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PAD_ESCAPE_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_PAD_ESCAPE_SYMBOL */
    #[\Since('8.4')]
    public const int PAD_ESCAPE_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_INFINITY_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const INFINITY_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_INFINITY_SYMBOL */
    #[\Since('8.4')]
    public const int INFINITY_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_NAN_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NAN_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_NAN_SYMBOL */
    #[\Since('8.4')]
    public const int NAN_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_SIGNIFICANT_DIGIT_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SIGNIFICANT_DIGIT_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_SIGNIFICANT_DIGIT_SYMBOL */
    #[\Since('8.4')]
    public const int SIGNIFICANT_DIGIT_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MONETARY_GROUPING_SEPARATOR_SYMBOL = UNKNOWN;
    /** @cvalue UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL */
    #[\Since('8.4')]
    public const int MONETARY_GROUPING_SEPARATOR_SYMBOL = UNKNOWN;
    /**
     * @var int
     * @cvalue FORMAT_TYPE_DEFAULT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_DEFAULT = UNKNOWN;
    /** @cvalue FORMAT_TYPE_DEFAULT */
    #[\Since('8.4')]
    public const int TYPE_DEFAULT = UNKNOWN;
    /**
     * @var int
     * @cvalue FORMAT_TYPE_INT32
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_INT32 = UNKNOWN;
    /** @cvalue FORMAT_TYPE_INT32 */
    #[\Since('8.4')]
    public const int TYPE_INT32 = UNKNOWN;
    /**
     * @var int
     * @cvalue FORMAT_TYPE_INT64
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_INT64 = UNKNOWN;
    /** @cvalue FORMAT_TYPE_INT64 */
    #[\Since('8.4')]
    public const int TYPE_INT64 = UNKNOWN;
    /**
     * @var int
     * @cvalue FORMAT_TYPE_DOUBLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_DOUBLE = UNKNOWN;
    /** @cvalue FORMAT_TYPE_DOUBLE */
    #[\Since('8.4')]
    public const int TYPE_DOUBLE = UNKNOWN;
    /**
     * @var int
     * @deprecated
     * @cvalue FORMAT_TYPE_CURRENCY
     */
    #[\Since('8.3')]
    #[\Until('8.4')]
    public const TYPE_CURRENCY = UNKNOWN;
    /**
     * @cvalue FORMAT_TYPE_CURRENCY
     */
    #[\Deprecated(since: '8.3')]
    #[\Since('8.4')]
    public const int TYPE_CURRENCY = UNKNOWN;
}