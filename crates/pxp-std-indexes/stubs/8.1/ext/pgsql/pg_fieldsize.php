<?php

/**
 * @param resource $result
 * @alias pg_field_size
 * @deprecated
 */
function pg_fieldsize($result, int $field): int
{
}
/**
 * @alias pg_field_size
 * @deprecated
 */
function pg_fieldsize(\PgSql\Result $result, int $field): int
{
}