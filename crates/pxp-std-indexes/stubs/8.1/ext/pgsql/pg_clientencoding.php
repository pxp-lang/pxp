<?php

/**
 * @param resource|null $connection
 * @alias pg_client_encoding
 * @deprecated
 */
function pg_clientencoding($connection = null): string
{
}
/**
 * @alias pg_client_encoding
 * @deprecated
 */
function pg_clientencoding(?\PgSql\Connection $connection = null): string
{
}