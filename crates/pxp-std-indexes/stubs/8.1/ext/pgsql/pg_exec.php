<?php

/**
 * @param resource|string $connection
 * @return resource|false
 * @alias pg_query
 */
function pg_exec($connection, string $query = UNKNOWN)
{
}
/**
 * @param PgSql\Connection|string $connection
 * @alias pg_query
 */
function pg_exec($connection, string $query = UNKNOWN): \PgSql\Result|false
{
}