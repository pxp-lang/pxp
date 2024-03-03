<?php

/**
 * @return array<int, string>|false
 * @refcount 1
 */
function pg_copy_to(\PgSql\Connection $connection, string $table_name, string $separator = "\t", string $null_as = "\\\\N"): array|false
{
}