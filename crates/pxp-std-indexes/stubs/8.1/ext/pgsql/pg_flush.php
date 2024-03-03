<?php

/** @param resource $connection */
function pg_flush($connection): int|bool
{
}
function pg_flush(\PgSql\Connection $connection): int|bool
{
}