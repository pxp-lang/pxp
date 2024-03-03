<?php

/**
 * @return array<int|string, int|string>
 * @refcount 1
 */
function pg_get_notify(\PgSql\Connection $connection, int $mode = PGSQL_ASSOC): array|false
{
}