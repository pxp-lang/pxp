<?php

/** @param resource $connection */
function pg_update($connection, string $table_name, array $values, array $conditions, int $flags = PGSQL_DML_EXEC): string|bool
{
}
/** @refcount 1 */
function pg_update(\PgSql\Connection $connection, string $table_name, array $values, array $conditions, int $flags = PGSQL_DML_EXEC): string|bool
{
}