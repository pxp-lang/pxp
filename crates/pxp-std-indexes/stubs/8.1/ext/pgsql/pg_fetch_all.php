<?php

/** @param resource $result */
function pg_fetch_all($result, int $mode = PGSQL_ASSOC): array
{
}
/**
 * @return array<int, array>
 * @refcount 1
 */
function pg_fetch_all(\PgSql\Result $result, int $mode = PGSQL_ASSOC): array
{
}