<?php

/** @param resource|null $connection */
function pg_ping($connection = null): bool
{
}
function pg_ping(?\PgSql\Connection $connection = null): bool
{
}