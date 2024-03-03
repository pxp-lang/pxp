<?php

/** @param resource $connection */
function pg_last_notice($connection, int $mode = PGSQL_NOTICE_LAST): array|string|bool
{
}
function pg_last_notice(\PgSql\Connection $connection, int $mode = PGSQL_NOTICE_LAST): array|string|bool
{
}