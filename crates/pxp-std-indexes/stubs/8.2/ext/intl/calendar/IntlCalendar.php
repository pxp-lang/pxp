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
     * @return bool
     * @alias intlcal_clear
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
     * @return bool
     * @alias intlcal_set_minimal_days_in_first_week
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
     * @return bool
     * @alias intlcal_set
     */
    public function set(int $year, int $month, int $dayOfMonth = UNKNOWN, int $hour = UNKNOWN, int $minute = UNKNOWN, int $second = UNKNOWN)
    {
    }
    /**
     * @return bool
     * @alias intlcal_set_first_day_of_week
     */
    public function setFirstDayOfWeek(int $dayOfWeek)
    {
    }
    /**
     * @return bool
     * @alias intlcal_set_lenient
     */
    public function setLenient(bool $lenient)
    {
    }
    /**
     * @return bool
     * @alias intlcal_set_repeated_wall_time_option
     */
    public function setRepeatedWallTimeOption(int $option)
    {
    }
    /**
     * @return bool
     * @alias intlcal_set_skipped_wall_time_option
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
    public const FIELD_ERA = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_YEAR
     */
    public const FIELD_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MONTH
     */
    public const FIELD_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEK_OF_YEAR
     */
    public const FIELD_WEEK_OF_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEK_OF_MONTH
     */
    public const FIELD_WEEK_OF_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DATE
     */
    public const FIELD_DATE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_YEAR
     */
    public const FIELD_DAY_OF_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_WEEK
     */
    public const FIELD_DAY_OF_WEEK = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_WEEK_IN_MONTH
     */
    public const FIELD_DAY_OF_WEEK_IN_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_AM_PM
     */
    public const FIELD_AM_PM = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_HOUR
     */
    public const FIELD_HOUR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_HOUR_OF_DAY
     */
    public const FIELD_HOUR_OF_DAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MINUTE
     */
    public const FIELD_MINUTE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_SECOND
     */
    public const FIELD_SECOND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MILLISECOND
     */
    public const FIELD_MILLISECOND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_ZONE_OFFSET
     */
    public const FIELD_ZONE_OFFSET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DST_OFFSET
     */
    public const FIELD_DST_OFFSET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_YEAR_WOY
     */
    public const FIELD_YEAR_WOY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DOW_LOCAL
     */
    public const FIELD_DOW_LOCAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_EXTENDED_YEAR
     */
    public const FIELD_EXTENDED_YEAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_JULIAN_DAY
     */
    public const FIELD_JULIAN_DAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MILLISECONDS_IN_DAY
     */
    public const FIELD_MILLISECONDS_IN_DAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_IS_LEAP_MONTH
     */
    public const FIELD_IS_LEAP_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_FIELD_COUNT
     */
    public const FIELD_FIELD_COUNT = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_DAY_OF_MONTH
     */
    public const FIELD_DAY_OF_MONTH = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_SUNDAY
     */
    public const DOW_SUNDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_MONDAY
     */
    public const DOW_MONDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_TUESDAY
     */
    public const DOW_TUESDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEDNESDAY
     */
    public const DOW_WEDNESDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_THURSDAY
     */
    public const DOW_THURSDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_FRIDAY
     */
    public const DOW_FRIDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_SATURDAY
     */
    public const DOW_SATURDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKDAY
     */
    public const DOW_TYPE_WEEKDAY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKEND
     */
    public const DOW_TYPE_WEEKEND = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKEND_ONSET
     */
    public const DOW_TYPE_WEEKEND_OFFSET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WEEKEND_CEASE
     */
    public const DOW_TYPE_WEEKEND_CEASE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WALLTIME_FIRST
     */
    public const WALLTIME_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WALLTIME_LAST
     */
    public const WALLTIME_LAST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_WALLTIME_NEXT_VALID
     */
    public const WALLTIME_NEXT_VALID = UNKNOWN;
}