<?php

/** @param resource $lob */
function pg_lo_close($lob): bool
{
}
function pg_lo_close(\PgSql\Lob $lob): bool
{
}