<?php

/**
 * @param resource $result
 * @alias pg_field_num
 * @deprecated
 */
function pg_fieldnum($result, string $field): int
{
}
/**
 * @alias pg_field_num
 * @deprecated
 */
function pg_fieldnum(\PgSql\Result $result, string $field): int
{
}