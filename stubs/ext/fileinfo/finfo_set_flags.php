<?php 

/**
 * @param resource $finfo
 */
#[\Until('8.1')]
function finfo_set_flags($finfo, int $flags): bool
{
}
#[\Since('8.1')]
#[\Until('8.4')]
function finfo_set_flags(\finfo $finfo, int $flags): bool
{
}
#[\Since('8.4')]
function finfo_set_flags(\finfo $finfo, int $flags): true
{
}