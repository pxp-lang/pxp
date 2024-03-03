<?php

/** @param resource|null $connection */
function pg_host($connection = null): string
{
}
/** @refcount 1 */
function pg_host(?\PgSql\Connection $connection = null): string
{
}