<?php

/**
 * @param resource $result
 * @alias pg_free_result
 * @deprecated
 */
function pg_freeresult($result): bool
{
}
/**
 * @alias pg_free_result
 * @deprecated
 */
function pg_freeresult(\PgSql\Result $result): bool
{
}