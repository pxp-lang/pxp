<?php

/**
 * @param resource $result
 * @param string|int $row
 */
function pg_field_is_null($result, $row, string|int $field = UNKNOWN): int|false
{
}
/** @param string|int $row */
function pg_field_is_null(\PgSql\Result $result, $row, string|int $field = UNKNOWN): int|false
{
}