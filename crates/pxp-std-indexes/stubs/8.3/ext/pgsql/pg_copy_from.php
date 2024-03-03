<?php

function pg_copy_from(\PgSql\Connection $connection, string $table_name, array $rows, string $separator = "\t", string $null_as = "\\\\N"): bool
{
}