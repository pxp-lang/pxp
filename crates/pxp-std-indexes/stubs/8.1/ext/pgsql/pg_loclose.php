<?php

/**
 * @param resource $lob
 * @alias pg_lo_close
 * @deprecated
 */
function pg_loclose($lob): bool
{
}
/**
 * @alias pg_lo_close
 * @deprecated
 */
function pg_loclose(\PgSql\Lob $lob): bool
{
}