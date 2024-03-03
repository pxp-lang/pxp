<?php

/** @param resource $connection */
function pg_meta_data($connection, string $table_name, bool $extended = false): array|false
{
}
/**
 * @return array<string, array>|false
 * @refcount 1
 */
function pg_meta_data(\PgSql\Connection $connection, string $table_name, bool $extended = false): array|false
{
}