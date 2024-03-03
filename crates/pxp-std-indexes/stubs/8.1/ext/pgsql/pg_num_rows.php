<?php

/** @param resource $result */
function pg_num_rows($result): int
{
}
function pg_num_rows(\PgSql\Result $result): int
{
}