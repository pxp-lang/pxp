<?php

/**
 * @param string|int $row
 * @alias pg_fetch_result
 * @deprecated
 */
function pg_result(\PgSql\Result $result, $row, string|int $field = UNKNOWN): string|false|null
{
}