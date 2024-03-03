<?php

/** @param resource $result */
function pg_result_seek($result, int $row): bool
{
}
function pg_result_seek(\PgSql\Result $result, int $row): bool
{
}