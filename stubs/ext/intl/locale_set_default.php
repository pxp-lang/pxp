<?php 

#[\Until('8.4')]
function locale_set_default(string $locale): bool
{
}
#[\Since('8.4')]
function locale_set_default(string $locale): true
{
}