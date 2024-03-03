<?php

/** @param resource $lob */
function pg_lo_tell($lob): int
{
}
function pg_lo_tell(\PgSql\Lob $lob): int
{
}