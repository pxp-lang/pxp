<?php

/**
 * @param resource $connection
 * @return resource|string|bool
 */
function pg_insert($connection, string $table_name, array $values, int $flags = PGSQL_DML_EXEC)
{
}