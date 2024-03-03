<?php

/** @param resource $result */
function pg_result_error($result): string|false
{
}
/** @refcount 1 */
function pg_result_error(\PgSql\Result $result): string|false
{
}