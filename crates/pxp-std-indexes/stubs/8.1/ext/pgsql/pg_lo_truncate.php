<?php

/** @param resource $lob */
function pg_lo_truncate($lob, int $size): bool
{
}
function pg_lo_truncate(\PgSql\Lob $lob, int $size): bool
{
}