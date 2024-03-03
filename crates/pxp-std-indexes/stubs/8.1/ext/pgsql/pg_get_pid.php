<?php

/** @param resource $connection */
function pg_get_pid($connection): int
{
}
function pg_get_pid(\PgSql\Connection $connection): int
{
}