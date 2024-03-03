<?php

/** @param resource $connection */
function pg_transaction_status($connection): int
{
}
function pg_transaction_status(\PgSql\Connection $connection): int
{
}