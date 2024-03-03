<?php

/**
 * @param resource $lob
 * @alias pg_lo_read_all
 * @deprecated
 */
function pg_loreadall($lob): int
{
}
/**
 * @alias pg_lo_read_all
 * @deprecated
 */
function pg_loreadall(\PgSql\Lob $lob): int
{
}