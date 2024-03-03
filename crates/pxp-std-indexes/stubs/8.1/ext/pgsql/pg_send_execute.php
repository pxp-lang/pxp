<?php

/** @param resource $connection */
function pg_send_execute($connection, string $statement_name, array $params): int|bool
{
}
function pg_send_execute(\PgSql\Connection $connection, string $statement_name, array $params): int|bool
{
}