<?php

/**
 * @param resource $connection
 * @return resource|false
 */
function pg_socket($connection)
{
}
/**
 * @return resource|false
 * @refcount 1
 */
function pg_socket(\PgSql\Connection $connection)
{
}