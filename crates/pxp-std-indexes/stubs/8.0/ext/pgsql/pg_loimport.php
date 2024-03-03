<?php

/**
 * @param resource|string $connection
 * @param string|int $filename
 * @param string|int $oid
 * @return resource|false
 * @alias pg_lo_import
 * @deprecated
 */
function pg_loimport($connection, $filename = UNKNOWN, $oid = UNKNOWN): string|int|false
{
}