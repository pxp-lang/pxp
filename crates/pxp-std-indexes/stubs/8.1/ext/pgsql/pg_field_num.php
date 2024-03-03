<?php

/** @param resource $result */
function pg_field_num($result, string $field): int
{
}
function pg_field_num(\PgSql\Result $result, string $field): int
{
}