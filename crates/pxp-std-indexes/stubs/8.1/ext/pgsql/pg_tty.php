<?php

/** @param resource|null $connection */
function pg_tty($connection = null): string
{
}
/** @refcount 1 */
function pg_tty(?\PgSql\Connection $connection = null): string
{
}