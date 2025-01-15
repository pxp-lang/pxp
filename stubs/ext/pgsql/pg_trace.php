<?php 

/** @param resource|null $connection */
#[\Until('8.1')]
function pg_trace(string $filename, string $mode = "w", $connection = null): bool
{
}
#[\Since('8.1')]
#[\Until('8.3')]
function pg_trace(string $filename, string $mode = "w", ?\PgSql\Connection $connection = null): bool
{
}
#[\Since('8.3')]
function pg_trace(string $filename, string $mode = "w", ?\PgSql\Connection $connection = null, int $trace_mode = 0): bool
{
}