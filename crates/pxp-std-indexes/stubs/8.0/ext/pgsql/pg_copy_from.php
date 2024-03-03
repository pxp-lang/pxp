<?php

/** @param resource $connection */
function pg_copy_from($connection, string $table_name, array $rows, string $separator = "\t", string $null_as = "\\\\N"): bool
{
}