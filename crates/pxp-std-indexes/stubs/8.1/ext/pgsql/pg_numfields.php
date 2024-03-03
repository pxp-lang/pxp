<?php

/**
 * @param resource $result
 * @alias pg_num_fields
 * @deprecated
 */
function pg_numfields($result): int
{
}
/**
 * @alias pg_num_fields
 * @deprecated
 */
function pg_numfields(\PgSql\Result $result): int
{
}