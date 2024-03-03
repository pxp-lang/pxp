<?php

/**
 * @param PgSql\Connection|string $connection
 * @alias pg_set_client_encoding
 * @deprecated
 */
function pg_setclientencoding($connection, string $encoding = UNKNOWN): int
{
}