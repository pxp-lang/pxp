<?php

/** @param resource $connection */
function pg_update($connection, string $table_name, array $values, array $conditions, int $flags = PGSQL_DML_EXEC): string|bool
{
}