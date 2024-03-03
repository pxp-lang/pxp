<?php

/** @param resource|null $connection */
function pg_close($connection = null): bool
{
}
function pg_close(?\PgSql\Connection $connection = null): bool
{
}