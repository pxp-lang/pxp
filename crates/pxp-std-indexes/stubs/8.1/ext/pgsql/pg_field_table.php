<?php

/** @param resource $result */
function pg_field_table($result, int $field, bool $oid_only = false): string|int|false
{
}
function pg_field_table(\PgSql\Result $result, int $field, bool $oid_only = false): string|int|false
{
}