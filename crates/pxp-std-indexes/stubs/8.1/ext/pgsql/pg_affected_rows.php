<?php

/** @param resource $result */
function pg_affected_rows($result): int
{
}
function pg_affected_rows(\PgSql\Result $result): int
{
}