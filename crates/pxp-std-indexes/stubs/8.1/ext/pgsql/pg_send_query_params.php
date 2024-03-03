<?php

/** @param resource $connection */
function pg_send_query_params($connection, string $query, array $params): int|bool
{
}
function pg_send_query_params(\PgSql\Connection $connection, string $query, array $params): int|bool
{
}