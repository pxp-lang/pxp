<?php

/** @param resource $connection */
function pg_connection_busy($connection): bool
{
}
function pg_connection_busy(\PgSql\Connection $connection): bool
{
}