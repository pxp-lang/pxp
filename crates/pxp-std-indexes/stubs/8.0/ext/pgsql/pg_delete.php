<?php

/** @param resource $connection */
function pg_delete($connection, string $table_name, array $conditions, int $flags = PGSQL_DML_EXEC): string|bool
{
}