<?php

/**
 * @param PgSql\Connection|string $connection
 * @param string|array $statement_name
 * @refcount 1
 */
function pg_execute($connection, $statement_name, array $params = UNKNOWN): \PgSql\Result|false
{
}