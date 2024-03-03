<?php

/** @param resource $connection */
function pg_connection_status($connection): int
{
}
function pg_connection_status(\PgSql\Connection $connection): int
{
}