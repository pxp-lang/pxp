<?php

/** @return resource|false */
function pg_pconnect(string $connection_string, int $flags = 0)
{
}
function pg_pconnect(string $connection_string, int $flags = 0): \PgSql\Connection|false
{
}