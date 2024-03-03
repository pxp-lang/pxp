<?php

/**
 * @param PgSql\Connection $connection
 * @param string|int $oid
 * @alias pg_lo_open
 * @deprecated
 */
function pg_loopen($connection, $oid = UNKNOWN, string $mode = UNKNOWN): \PgSql\Lob|false
{
}