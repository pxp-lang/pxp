<?php

/**
 * @param string|array $key
 * @param int|resource $skip actually this parameter is optional, not $dba
 * @param resource $dba
 */
function dba_fetch($key, $skip, $dba = UNKNOWN): string|false
{
}
/**
 * @param resource|int $dba overloaded legacy signature has params flipped
 * @param resource|int $skip overloaded legacy signature has params flipped
 */
function dba_fetch(string|array $key, $dba, $skip = 0): string|false
{
}