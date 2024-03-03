<?php

/** @param resource|null $connection */
function pg_port($connection = null): string
{
}
/** @refcount 1 */
function pg_port(?\PgSql\Connection $connection = null): string
{
}