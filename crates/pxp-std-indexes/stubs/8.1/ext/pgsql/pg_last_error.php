<?php

/** @param resource|null $connection */
function pg_last_error($connection = null): string
{
}
function pg_last_error(?\PgSql\Connection $connection = null): string
{
}