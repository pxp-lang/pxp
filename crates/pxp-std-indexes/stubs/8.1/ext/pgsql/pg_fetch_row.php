<?php

/**
 * @param resource $result
 */
function pg_fetch_row($result, ?int $row = null, int $mode = PGSQL_NUM): array|false
{
}
/**
 * @return array<int|string, string|null>|false
 * @refcount 1
 */
function pg_fetch_row(\PgSql\Result $result, ?int $row = null, int $mode = PGSQL_NUM): array|false
{
}