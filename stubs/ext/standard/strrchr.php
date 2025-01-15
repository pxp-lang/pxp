<?php 

#[\Until('8.3')]
function strrchr(string $haystack, string $needle): string|false
{
}
/**
 * @compile-time-eval
 * @refcount 1
 */
#[\Since('8.3')]
function strrchr(string $haystack, string $needle, bool $before_needle = false): string|false
{
}