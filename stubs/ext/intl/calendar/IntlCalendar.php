<?php 

/** @generate-function-entries */
class IntlCalendar
{
    private function __construct()
    {
    }
    /**
     * @param (IntlTimeZone | DateTimeZone | string | null) $timezone
     * @tentative-return-type
     * @alias intlcal_create_instance
     * @return (IntlCalendar | null)
     */
    public static function createInstance($timezone = null, ?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_equals
     * @return bool
     */
    public function equals(IntlCalendar $other)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_field_difference
     * @return (int | false)
     */
    public function fieldDifference(float $timestamp, int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_add
     * @return bool
     */
    public function add(int $field, int $value)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_after
     * @return bool
     */
    public function after(IntlCalendar $other)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_before
     * @return bool
     */
    public function before(IntlCalendar $other)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_clear
     * @return bool
     */
    public function clear(?int $field = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_from_date_time
     * @return (IntlCalendar | null)
     */
    public static function fromDateTime(DateTime|string $datetime, ?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get
     * @return (int | false)
     */
    public function get(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_actual_maximum
     * @return (int | false)
     */
    public function getActualMaximum(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_actual_minimum
     * @return (int | false)
     */
    public function getActualMinimum(int $field)
    {
    }
    /**
     * @return array
     * @alias intlcal_get_available_locales
     */
    public static function getAvailableLocales()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_day_of_week_type
     * @return (int | false)
     */
    public function getDayOfWeekType(int $dayOfWeek)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_error_code
     * @return (int | false)
     */
    public function getErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_error_message
     * @return (string | false)
     */
    public function getErrorMessage()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_first_day_of_week
     * @return (int | false)
     */
    public function getFirstDayOfWeek()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_greatest_minimum
     * @return (int | false)
     */
    public function getGreatestMinimum(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_keyword_values_for_locale
     * @return (IntlIterator | false)
     */
    public static function getKeywordValuesForLocale(string $keyword, string $locale, bool $onlyCommon)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_least_maximum
     * @return (int | false)
     */
    public function getLeastMaximum(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_locale
     * @return (string | false)
     */
    public function getLocale(int $type)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_maximum
     * @return (int | false)
     */
    public function getMaximum(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_minimal_days_in_first_week
     * @return (int | false)
     */
    public function getMinimalDaysInFirstWeek()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_set_minimal_days_in_first_week
     * @return bool
     */
    public function setMinimalDaysInFirstWeek(int $days)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_minimum
     * @return (int | false)
     */
    public function getMinimum(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_now
     * @return float
     */
    public static function getNow()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_repeated_wall_time_option
     * @return int
     */
    public function getRepeatedWallTimeOption()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_skipped_wall_time_option
     * @return int
     */
    public function getSkippedWallTimeOption()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_time
     * @return (float | false)
     */
    public function getTime()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_time_zone
     * @return (IntlTimeZone | false)
     */
    public function getTimeZone()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_type
     * @return string
     */
    public function getType()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_get_weekend_transition
     * @return (int | false)
     */
    public function getWeekendTransition(int $dayOfWeek)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_in_daylight_time
     * @return bool
     */
    public function inDaylightTime()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_is_equivalent_to
     * @return bool
     */
    public function isEquivalentTo(IntlCalendar $other)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_is_lenient
     * @return bool
     */
    public function isLenient()
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_is_weekend
     * @return bool
     */
    public function isWeekend(?float $timestamp = null)
    {
    }
    /**
     * @param (int | bool) $value
     * @alias intlcal_roll
     * @tentative-return-type
     * @return bool
     */
    public function roll(int $field, $value)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_is_set
     * @return bool
     */
    public function isSet(int $field)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias intlcal_set
     * @return bool
     */
    public function set(int $year, int $month, int $dayOfMonth = UNKNOWN, int $hour = UNKNOWN, int $minute = UNKNOWN, int $second = UNKNOWN)
    {
    }
    // TODO make return type void
    #[\Since('8.3')]
    public function setDate(int $year, int $month, int $dayOfMonth): void
    {
    }
    #[\Since('8.3')]
    public function setDateTime(int $year, int $month, int $dayOfMonth, int $hour, int $minute, ?int $second = null): void
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_set_first_day_of_week
     * @return bool
     */
    public function setFirstDayOfWeek(int $dayOfWeek)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_set_lenient
     * @return bool
     */
    public function setLenient(bool $lenient)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_set_repeated_wall_time_option
     * @return bool
     */
    public function setRepeatedWallTimeOption(int $option)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_set_skipped_wall_time_option
     * @return bool
     */
    public function setSkippedWallTimeOption(int $option)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_set_time
     * @return bool
     */
    public function setTime(float $timestamp)
    {
    }
    /**
     * @param (IntlTimeZone | DateTimeZone | string | null) $timezone
     * @tentative-return-type
     * @alias intlcal_set_time_zone
     * @return bool
     */
    public function setTimeZone($timezone)
    {
    }
    /**
     * @tentative-return-type
     * @alias intlcal_to_date_time
     * @return (DateTime | false)
     */
    public function toDateTime()
    {
    }
    /**
     * @var int
     * @cvalue UCAL_ERA
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_ERA = UNKNOWN;
    /** @cvalue UCAL_ERA */
    #[\Since('8.4')]
    public const int FIELD_ERA = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_YEAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_YEAR = UNKNOWN;
    /** @cvalue UCAL_YEAR */
    #[\Since('8.4')]
    public const int FIELD_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MONTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_MONTH = UNKNOWN;
    /** @cvalue UCAL_MONTH */
    #[\Since('8.4')]
    public const int FIELD_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEK_OF_YEAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_WEEK_OF_YEAR = UNKNOWN;
    /** @cvalue UCAL_WEEK_OF_YEAR */
    #[\Since('8.4')]
    public const int FIELD_WEEK_OF_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEK_OF_MONTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_WEEK_OF_MONTH = UNKNOWN;
    /** @cvalue UCAL_WEEK_OF_MONTH */
    #[\Since('8.4')]
    public const int FIELD_WEEK_OF_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DATE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DATE = UNKNOWN;
    /** @cvalue UCAL_DATE */
    #[\Since('8.4')]
    public const int FIELD_DATE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_YEAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DAY_OF_YEAR = UNKNOWN;
    /** @cvalue UCAL_DAY_OF_YEAR */
    #[\Since('8.4')]
    public const int FIELD_DAY_OF_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_WEEK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DAY_OF_WEEK = UNKNOWN;
    /** @cvalue UCAL_DAY_OF_WEEK */
    #[\Since('8.4')]
    public const int FIELD_DAY_OF_WEEK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_WEEK_IN_MONTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DAY_OF_WEEK_IN_MONTH = UNKNOWN;
    /** @cvalue UCAL_DAY_OF_WEEK_IN_MONTH */
    #[\Since('8.4')]
    public const int FIELD_DAY_OF_WEEK_IN_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_AM_PM
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_AM_PM = UNKNOWN;
    /** @cvalue UCAL_AM_PM */
    #[\Since('8.4')]
    public const int FIELD_AM_PM = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_HOUR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_HOUR = UNKNOWN;
    /** @cvalue UCAL_HOUR */
    #[\Since('8.4')]
    public const int FIELD_HOUR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_HOUR_OF_DAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_HOUR_OF_DAY = UNKNOWN;
    /** @cvalue UCAL_HOUR_OF_DAY */
    #[\Since('8.4')]
    public const int FIELD_HOUR_OF_DAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MINUTE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_MINUTE = UNKNOWN;
    /** @cvalue UCAL_MINUTE */
    #[\Since('8.4')]
    public const int FIELD_MINUTE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_SECOND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_SECOND = UNKNOWN;
    /** @cvalue UCAL_SECOND */
    #[\Since('8.4')]
    public const int FIELD_SECOND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MILLISECOND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_MILLISECOND = UNKNOWN;
    /** @cvalue UCAL_MILLISECOND */
    #[\Since('8.4')]
    public const int FIELD_MILLISECOND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_ZONE_OFFSET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_ZONE_OFFSET = UNKNOWN;
    /** @cvalue UCAL_ZONE_OFFSET */
    #[\Since('8.4')]
    public const int FIELD_ZONE_OFFSET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DST_OFFSET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DST_OFFSET = UNKNOWN;
    /** @cvalue UCAL_DST_OFFSET */
    #[\Since('8.4')]
    public const int FIELD_DST_OFFSET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_YEAR_WOY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_YEAR_WOY = UNKNOWN;
    /** @cvalue UCAL_YEAR_WOY */
    #[\Since('8.4')]
    public const int FIELD_YEAR_WOY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DOW_LOCAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DOW_LOCAL = UNKNOWN;
    /** @cvalue UCAL_DOW_LOCAL */
    #[\Since('8.4')]
    public const int FIELD_DOW_LOCAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_EXTENDED_YEAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_EXTENDED_YEAR = UNKNOWN;
    /** @cvalue UCAL_EXTENDED_YEAR */
    #[\Since('8.4')]
    public const int FIELD_EXTENDED_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_JULIAN_DAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_JULIAN_DAY = UNKNOWN;
    /** @cvalue UCAL_JULIAN_DAY */
    #[\Since('8.4')]
    public const int FIELD_JULIAN_DAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MILLISECONDS_IN_DAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_MILLISECONDS_IN_DAY = UNKNOWN;
    /** @cvalue UCAL_MILLISECONDS_IN_DAY */
    #[\Since('8.4')]
    public const int FIELD_MILLISECONDS_IN_DAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_IS_LEAP_MONTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_IS_LEAP_MONTH = UNKNOWN;
    /** @cvalue UCAL_IS_LEAP_MONTH */
    #[\Since('8.4')]
    public const int FIELD_IS_LEAP_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_FIELD_COUNT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_FIELD_COUNT = UNKNOWN;
    /** @cvalue UCAL_FIELD_COUNT */
    #[\Since('8.4')]
    public const int FIELD_FIELD_COUNT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_MONTH
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FIELD_DAY_OF_MONTH = UNKNOWN;
    /** @cvalue UCAL_DAY_OF_MONTH */
    #[\Since('8.4')]
    public const int FIELD_DAY_OF_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_SUNDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_SUNDAY = UNKNOWN;
    /** @cvalue UCAL_SUNDAY */
    #[\Since('8.4')]
    public const int DOW_SUNDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MONDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_MONDAY = UNKNOWN;
    /** @cvalue UCAL_MONDAY */
    #[\Since('8.4')]
    public const int DOW_MONDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_TUESDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_TUESDAY = UNKNOWN;
    /** @cvalue UCAL_TUESDAY */
    #[\Since('8.4')]
    public const int DOW_TUESDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEDNESDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_WEDNESDAY = UNKNOWN;
    /** @cvalue UCAL_WEDNESDAY */
    #[\Since('8.4')]
    public const int DOW_WEDNESDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_THURSDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_THURSDAY = UNKNOWN;
    /** @cvalue UCAL_THURSDAY */
    #[\Since('8.4')]
    public const int DOW_THURSDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_FRIDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_FRIDAY = UNKNOWN;
    /** @cvalue UCAL_FRIDAY */
    #[\Since('8.4')]
    public const int DOW_FRIDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_SATURDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_SATURDAY = UNKNOWN;
    /** @cvalue UCAL_SATURDAY */
    #[\Since('8.4')]
    public const int DOW_SATURDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKDAY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_TYPE_WEEKDAY = UNKNOWN;
    /** @cvalue UCAL_WEEKDAY */
    #[\Since('8.4')]
    public const int DOW_TYPE_WEEKDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKEND
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_TYPE_WEEKEND = UNKNOWN;
    /** @cvalue UCAL_WEEKEND */
    #[\Since('8.4')]
    public const int DOW_TYPE_WEEKEND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKEND_ONSET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_TYPE_WEEKEND_OFFSET = UNKNOWN;
    /** @cvalue UCAL_WEEKEND_ONSET */
    #[\Since('8.4')]
    public const int DOW_TYPE_WEEKEND_OFFSET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKEND_CEASE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DOW_TYPE_WEEKEND_CEASE = UNKNOWN;
    /** @cvalue UCAL_WEEKEND_CEASE */
    #[\Since('8.4')]
    public const int DOW_TYPE_WEEKEND_CEASE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WALLTIME_FIRST
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WALLTIME_FIRST = UNKNOWN;
    /** @cvalue UCAL_WALLTIME_FIRST */
    #[\Since('8.4')]
    public const int WALLTIME_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WALLTIME_LAST
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WALLTIME_LAST = UNKNOWN;
    /** @cvalue UCAL_WALLTIME_LAST */
    #[\Since('8.4')]
    public const int WALLTIME_LAST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WALLTIME_NEXT_VALID
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WALLTIME_NEXT_VALID = UNKNOWN;
    /** @cvalue UCAL_WALLTIME_NEXT_VALID */
    #[\Since('8.4')]
    public const int WALLTIME_NEXT_VALID = UNKNOWN;
}