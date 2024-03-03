<?php

/** @param resource|null $connection */
function pg_client_encoding($connection = null): string
{
}
function pg_client_encoding(?\PgSql\Connection $connection = null): string
{
}