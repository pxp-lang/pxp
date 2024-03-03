<?php

/* dateformat */
/** @param IntlTimeZone|DateTimeZone|string|null $timezone */
function datefmt_create(?string $locale, int $dateType, int $timeType, $timezone = null, \IntlCalendar|int|null $calendar = null, ?string $pattern = null): ?\IntlDateFormatter
{
}
/* dateformat */
/** @param IntlTimeZone|DateTimeZone|string|null $timezone */
function datefmt_create(?string $locale, int $dateType = IntlDateFormatter::FULL, int $timeType = IntlDateFormatter::FULL, $timezone = null, \IntlCalendar|int|null $calendar = null, ?string $pattern = null): ?\IntlDateFormatter
{
}