<?php

/**
 * @param PgSql\Connection|string $connection
 * @param string|int $filename
 * @param string|int $oid
 * @refcount 1
 */
function pg_lo_import($connection, $filename = UNKNOWN, $oid = UNKNOWN): string|int|false
{
}