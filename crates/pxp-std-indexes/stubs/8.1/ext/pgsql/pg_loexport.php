<?php

/**
 * @param resource|string|int $connection
 * @param string|int $oid
 * @param string|int $filename
 * @return resource|false
 * @alias pg_lo_export
 * @deprecated
 */
function pg_loexport($connection, $oid = UNKNOWN, $filename = UNKNOWN): bool
{
}
/**
 * @param PgSql\Connection|string|int $connection
 * @param string|int $oid
 * @param string|int $filename
 * @alias pg_lo_export
 * @deprecated
 */
function pg_loexport($connection, $oid = UNKNOWN, $filename = UNKNOWN): bool
{
}