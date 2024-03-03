<?php

/** @param resource $connection */
function pg_send_prepare($connection, string $statement_name, string $query): int|bool
{
}
function pg_send_prepare(\PgSql\Connection $connection, string $statement_name, string $query): int|bool
{
}