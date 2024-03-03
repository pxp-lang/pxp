<?php

/** @param resource $result */
function pg_field_name($result, int $field): string
{
}
/** @refcount 1 */
function pg_field_name(\PgSql\Result $result, int $field): string
{
}