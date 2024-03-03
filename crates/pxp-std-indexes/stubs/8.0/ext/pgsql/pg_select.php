<?php

/** @param resource $connection */
function pg_select($connection, string $table_name, array $conditions, int $flags = PGSQL_DML_EXEC, int $mode = PGSQL_ASSOC): array|string|false
{
}