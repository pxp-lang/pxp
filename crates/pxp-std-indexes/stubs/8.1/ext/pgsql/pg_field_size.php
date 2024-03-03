<?php

/** @param resource $result */
function pg_field_size($result, int $field): int
{
}
function pg_field_size(\PgSql\Result $result, int $field): int
{
}