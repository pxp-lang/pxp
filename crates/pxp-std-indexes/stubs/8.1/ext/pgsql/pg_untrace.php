<?php

/** @param resource|null $connection */
function pg_untrace($connection = null): bool
{
}
function pg_untrace(?\PgSql\Connection $connection = null): bool
{
}