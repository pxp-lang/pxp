<?php 

#[\Until('8.4')]
function highlight_string(string $string, bool $return = false): string|bool
{
}
/** @refcount 1 */
#[\Since('8.4')]
function highlight_string(string $string, bool $return = false): string|true
{
}