<?php

/**
 * @return array<int|string, string|null>|false
 * @refcount 1
 */
function pg_fetch_array(\PgSql\Result $result, ?int $row = null, int $mode = PGSQL_BOTH): array|false
{
}