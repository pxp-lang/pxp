<?php

/** @param resource $connection */
function pg_delete($connection, string $table_name, array $conditions, int $flags = PGSQL_DML_EXEC): string|bool
{
}
/** @refcount 1 */
function pg_delete(\PgSql\Connection $connection, string $table_name, array $conditions, int $flags = PGSQL_DML_EXEC): string|bool
{
}