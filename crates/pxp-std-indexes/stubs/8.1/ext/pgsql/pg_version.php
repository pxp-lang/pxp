<?php

/** @param resource|null $connection */
function pg_version($connection = null): array
{
}
/**
 * @return array<string, int|string|null>
 * @refcount 1
 */
function pg_version(?\PgSql\Connection $connection = null): array
{
}