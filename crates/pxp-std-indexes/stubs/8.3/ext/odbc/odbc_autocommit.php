<?php

/** @param resource $odbc */
function odbc_autocommit($odbc, bool $enable = false): int|bool
{
}
/** @param resource $odbc */
function odbc_autocommit($odbc, ?bool $enable = null): int|bool
{
}