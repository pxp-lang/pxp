<?php

/** @param resource $lob */
function pg_lo_write($lob, string $data, ?int $length = null): int|false
{
}
function pg_lo_write(\PgSql\Lob $lob, string $data, ?int $length = null): int|false
{
}