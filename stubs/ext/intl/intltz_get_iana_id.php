<?php 

#if U_ICU_VERSION_MAJOR_NUM >= 74
#[\Since('8.4')]
function intltz_get_iana_id(string $timezoneId): string|false
{
}