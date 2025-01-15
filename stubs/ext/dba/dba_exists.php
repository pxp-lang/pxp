<?php 

/**
 * @param string|array $key
 * @param resource $dba
 */
#[\Until('8.2')]
function dba_exists($key, $dba): bool
{
}
/** @param resource $dba */
#[\Since('8.2')]
#[\Until('8.4')]
function dba_exists(string|array $key, $dba): bool
{
}
#[\Since('8.4')]
function dba_exists(string|array $key, \Dba\Connection $dba): bool
{
}