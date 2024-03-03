<?php

/**
 * @param PgSql\Connection|string $connection
 * @refcount 1
 */
function pg_prepare($connection, string $statement_name, string $query = UNKNOWN): \PgSql\Result|false
{
}