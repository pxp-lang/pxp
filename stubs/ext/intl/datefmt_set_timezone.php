<?php 

/** @param IntlTimeZone|DateTimeZone|string|null $timezone */
#[\Until('8.3')]
function datefmt_set_timezone(\IntlDateFormatter $formatter, $timezone): ?bool
{
}
/** @param IntlTimeZone|DateTimeZone|string|null $timezone */
#[\Since('8.3')]
function datefmt_set_timezone(\IntlDateFormatter $formatter, $timezone): bool
{
}