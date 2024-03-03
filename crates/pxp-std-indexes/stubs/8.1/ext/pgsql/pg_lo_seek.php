<?php

/** @param resource $lob */
function pg_lo_seek($lob, int $offset, int $whence = SEEK_CUR): bool
{
}
function pg_lo_seek(\PgSql\Lob $lob, int $offset, int $whence = SEEK_CUR): bool
{
}