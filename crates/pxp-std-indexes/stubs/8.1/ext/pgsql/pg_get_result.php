<?php

/**
 * @param resource $connection
 * @return resource|false
 */
function pg_get_result($connection)
{
}
/** @refcount 1 */
function pg_get_result(\PgSql\Connection $connection): \PgSql\Result|false
{
}