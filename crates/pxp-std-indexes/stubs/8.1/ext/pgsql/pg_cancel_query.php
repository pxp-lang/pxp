<?php

/** @param resource $connection */
function pg_cancel_query($connection): bool
{
}
function pg_cancel_query(\PgSql\Connection $connection): bool
{
}