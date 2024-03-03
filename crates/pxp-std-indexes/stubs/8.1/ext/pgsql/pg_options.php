<?php

/** @param resource|null $connection */
function pg_options($connection = null): string
{
}
/** @refcount 1 */
function pg_options(?\PgSql\Connection $connection = null): string
{
}