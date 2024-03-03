<?php

/**
 * @param string|int $row
 * @refcount 1
 */
function pg_fetch_result(\PgSql\Result $result, $row, string|int $field = UNKNOWN): string|false|null
{
}