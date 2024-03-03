<?php

/** @param resource $result */
function pg_field_type_oid($result, int $field): string|int
{
}
/** @refcount 1 */
function pg_field_type_oid(\PgSql\Result $result, int $field): string|int
{
}