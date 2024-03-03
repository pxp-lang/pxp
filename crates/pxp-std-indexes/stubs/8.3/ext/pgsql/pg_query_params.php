<?php

/**
 * @param PgSql\Connection|string $connection
 * @param string|array $query
 * @refcount 1
 */
function pg_query_params($connection, $query, array $params = UNKNOWN): \PgSql\Result|false
{
}