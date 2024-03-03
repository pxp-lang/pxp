<?php

/** @param resource $result */
function pg_last_oid($result): string|int|false
{
}
/** @refcount 1 */
function pg_last_oid(\PgSql\Result $result): string|int|false
{
}