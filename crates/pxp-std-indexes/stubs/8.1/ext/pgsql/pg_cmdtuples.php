<?php

/**
 * @param resource $result
 * @alias pg_affected_rows
 * @deprecated
 */
function pg_cmdtuples($result): int
{
}
/**
 * @alias pg_affected_rows
 * @deprecated
 */
function pg_cmdtuples(\PgSql\Result $result): int
{
}