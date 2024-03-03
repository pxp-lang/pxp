<?php

/**
 * @param resource $result
 * @param string|int $row
 */
function pg_field_prtlen($result, $row, string|int $field = UNKNOWN): int|false
{
}
/** @param string|int $row */
function pg_field_prtlen(\PgSql\Result $result, $row, string|int $field = UNKNOWN): int|false
{
}