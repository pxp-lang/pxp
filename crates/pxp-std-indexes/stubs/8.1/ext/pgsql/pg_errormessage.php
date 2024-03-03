<?php

/**
 * @param resource|null $connection
 * @alias pg_last_error
 * @deprecated
 */
function pg_errormessage($connection = null): string
{
}
/**
 * @alias pg_last_error
 * @deprecated
 */
function pg_errormessage(?\PgSql\Connection $connection = null): string
{
}