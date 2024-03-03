<?php

/** @param resource|null $connection */
function pg_trace(string $filename, string $mode = "w", $connection = null): bool
{
}
function pg_trace(string $filename, string $mode = "w", ?\PgSql\Connection $connection = null): bool
{
}