<?php

/** @param resource $result */
function pg_field_type($result, int $field): string
{
}
function pg_field_type(\PgSql\Result $result, int $field): string
{
}