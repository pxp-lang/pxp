<?php 

#[\Until('8.3')]
function intlcal_clear(\IntlCalendar $calendar, ?int $field = null): bool
{
}
#[\Since('8.3')]
function intlcal_clear(\IntlCalendar $calendar, ?int $field = null): true
{
}