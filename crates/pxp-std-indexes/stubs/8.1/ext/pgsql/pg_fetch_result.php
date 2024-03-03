<?php

/**
 * @param resource $result
 * @param string|int $row
 */
function pg_fetch_result($result, $row, string|int $field = UNKNOWN): string|false|null
{
}
/**
 * @param string|int $row
 * @refcount 1
 */
function pg_fetch_result(\PgSql\Result $result, $row, string|int $field = UNKNOWN): string|false|null
{
}