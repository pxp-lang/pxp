<?php

/* dateformat */
/** @param IntlTimeZone|DateTimeZone|string|null $timezone */
function datefmt_create(?string $locale, int $dateType, int $timeType, $timezone = null, \IntlCalendar|int|null $calendar = null, ?string $pattern = null): ?\IntlDateFormatter
{
}