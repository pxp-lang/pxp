<?php

/** @param resource|null $connection */
function pg_dbname($connection = null): string
{
}
/** @refcount 1 */
function pg_dbname(?\PgSql\Connection $connection = null): string
{
}