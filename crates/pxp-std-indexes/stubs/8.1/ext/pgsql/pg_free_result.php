<?php

/** @param resource $result */
function pg_free_result($result): bool
{
}
function pg_free_result(\PgSql\Result $result): bool
{
}