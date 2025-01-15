<?php 

/** @generate-function-entries */
class IntlTimeZone
{
    private function __construct()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_count_equivalent_ids
     * @return (int | false)
     */
    public static function countEquivalentIDs(string $timezoneId)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_create_default
     * @return IntlTimeZone
     */
    public static function createDefault()
    {
    }
    /**
     * @param (IntlTimeZone | string | int | float | null) $countryOrRawOffset
     * @tentative-return-type
     * @alias intltz_create_enumeration
     * @return (IntlIterator | false)
     */
    public static function createEnumeration($countryOrRawOffset = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_create_time_zone
     * @return (IntlTimeZone | null)
     */
    public static function createTimeZone(string $timezoneId)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_create_time_zone_id_enumeration
     * @return (IntlIterator | false)
     */
    public static function createTimeZoneIDEnumeration(int $type, ?string $region = null, ?int $rawOffset = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_from_date_time_zone
     * @return (IntlTimeZone | null)
     */
    public static function fromDateTimeZone(DateTimeZone $timezone)
    {
    }
    /**
     * @param bool $isSystemId
     * @tentative-return-type
     * @alias intltz_get_canonical_id
     * @return (string | false)
     */
    public static function getCanonicalID(string $timezoneId, &$isSystemId = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_display_name
     * @return (string | false)
     */
    public function getDisplayName(bool $dst = false, int $style = IntlTimeZone::DISPLAY_LONG, ?string $locale = null)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_dst_savings
     * @return int
     */
    public function getDSTSavings()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_equivalent_id
     * @return (string | false)
     */
    public static function getEquivalentID(string $timezoneId, int $offset)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_error_code
     * @return (int | false)
     */
    public function getErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_error_message
     * @return (string | false)
     */
    public function getErrorMessage()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_gmt
     * @return IntlTimeZone
     */
    public static function getGMT()
    {
    }
    #if U_ICU_VERSION_MAJOR_NUM >= 74
    /**
     * @alias intltz_get_iana_id
     */
    #[\Since('8.4')]
    public static function getIanaID(string $timezoneId): string|false
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_id
     * @return (string | false)
     */
    public function getID()
    {
    }
    /**
     * @param int $rawOffset
     * @param int $dstOffset
     * @tentative-return-type
     * @alias intltz_get_offset
     * @return bool
     */
    public function getOffset(float $timestamp, bool $local, &$rawOffset, &$dstOffset)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_raw_offset
     * @return int
     */
    public function getRawOffset()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_region
     * @return (string | false)
     */
    public static function getRegion(string $timezoneId)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_tz_data_version
     * @return (string | false)
     */
    public static function getTZDataVersion()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_unknown
     * @return IntlTimeZone
     */
    public static function getUnknown()
    {
    }
    #if U_ICU_VERSION_MAJOR_NUM >= 52
    /**
     * @tentative-return-type
     * @alias intltz_get_windows_id
     * @return (string | false)
     */
    public static function getWindowsID(string $timezoneId)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_get_id_for_windows_id
     * @return (string | false)
     */
    public static function getIDForWindowsID(string $timezoneId, ?string $region = null)
    {
    }
    #endif
    /**
     * @tentative-return-type
     * @alias intltz_has_same_rules
     * @return bool
     */
    public function hasSameRules(IntlTimeZone $other)
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_to_date_time_zone
     * @return (DateTimeZone | false)
     */
    public function toDateTimeZone()
    {
    }
    /**
     * @tentative-return-type
     * @alias intltz_use_daylight_time
     * @return bool
     */
    public function useDaylightTime()
    {
    }
    /**
     * @var int
     * @cvalue TimeZone::SHORT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_SHORT = UNKNOWN;
    /** @cvalue TimeZone::SHORT */
    #[\Since('8.4')]
    public const int DISPLAY_SHORT = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::LONG
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_LONG = UNKNOWN;
    /** @cvalue TimeZone::LONG */
    #[\Since('8.4')]
    public const int DISPLAY_LONG = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::SHORT_GENERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_SHORT_GENERIC = UNKNOWN;
    /** @cvalue TimeZone::SHORT_GENERIC */
    #[\Since('8.4')]
    public const int DISPLAY_SHORT_GENERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::LONG_GENERIC
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_LONG_GENERIC = UNKNOWN;
    /** @cvalue TimeZone::LONG_GENERIC */
    #[\Since('8.4')]
    public const int DISPLAY_LONG_GENERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::SHORT_GMT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_SHORT_GMT = UNKNOWN;
    /** @cvalue TimeZone::SHORT_GMT */
    #[\Since('8.4')]
    public const int DISPLAY_SHORT_GMT = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::LONG_GMT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_LONG_GMT = UNKNOWN;
    /** @cvalue TimeZone::LONG_GMT */
    #[\Since('8.4')]
    public const int DISPLAY_LONG_GMT = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::SHORT_COMMONLY_USED
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_SHORT_COMMONLY_USED = UNKNOWN;
    /** @cvalue TimeZone::SHORT_COMMONLY_USED */
    #[\Since('8.4')]
    public const int DISPLAY_SHORT_COMMONLY_USED = UNKNOWN;
    /**
     * @var int
     * @cvalue TimeZone::GENERIC_LOCATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DISPLAY_GENERIC_LOCATION = UNKNOWN;
    /** @cvalue TimeZone::GENERIC_LOCATION */
    #[\Since('8.4')]
    public const int DISPLAY_GENERIC_LOCATION = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_ZONE_TYPE_ANY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_ANY = UNKNOWN;
    /** @cvalue UCAL_ZONE_TYPE_ANY */
    #[\Since('8.4')]
    public const int TYPE_ANY = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_ZONE_TYPE_CANONICAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_CANONICAL = UNKNOWN;
    /** @cvalue UCAL_ZONE_TYPE_CANONICAL */
    #[\Since('8.4')]
    public const int TYPE_CANONICAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCAL_ZONE_TYPE_CANONICAL_LOCATION
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TYPE_CANONICAL_LOCATION = UNKNOWN;
    /** @cvalue UCAL_ZONE_TYPE_CANONICAL_LOCATION */
    #[\Since('8.4')]
    public const int TYPE_CANONICAL_LOCATION = UNKNOWN;
}