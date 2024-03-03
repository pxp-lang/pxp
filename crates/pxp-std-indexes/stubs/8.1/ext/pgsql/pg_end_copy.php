<?php

/** @param resource|null $connection */
function pg_end_copy($connection = null): bool
{
}
function pg_end_copy(?\PgSql\Connection $connection = null): bool
{
}