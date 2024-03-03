<?php

/** @param resource $result */
function pg_result_status($result, int $mode = PGSQL_STATUS_LONG): string|int
{
}
/** @refcount 1 */
function pg_result_status(\PgSql\Result $result, int $mode = PGSQL_STATUS_LONG): string|int
{
}