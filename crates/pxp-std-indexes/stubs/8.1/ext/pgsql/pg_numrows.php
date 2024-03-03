<?php

/**
 * @param resource $result
 * @alias pg_num_rows
 * @deprecated
 */
function pg_numrows($result): int
{
}
/**
 * @alias pg_num_rows
 * @deprecated
 */
function pg_numrows(\PgSql\Result $result): int
{
}