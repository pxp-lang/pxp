<?php

/** @param resource $connection */
function pg_send_query($connection, string $query): int|bool
{
}
function pg_send_query(\PgSql\Connection $connection, string $query): int|bool
{
}