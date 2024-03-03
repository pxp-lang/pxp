<?php

/**
 * @param resource $lob
 * @alias pg_lo_read
 * @deprecated
 */
function pg_loread($lob, int $length = 8192): string|false
{
}
/**
 * @alias pg_lo_read
 * @deprecated
 */
function pg_loread(\PgSql\Lob $lob, int $length = 8192): string|false
{
}