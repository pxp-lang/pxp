<?php 

/** @param resource $odbc */
#[\Until('8.3')]
function odbc_autocommit($odbc, bool $enable = false): int|bool
{
}
/** @param resource $odbc */
#[\Since('8.3')]
#[\Until('8.4')]
function odbc_autocommit($odbc, ?bool $enable = null): int|bool
{
}
#[\Since('8.4')]
function odbc_autocommit(\Odbc\Connection $odbc, ?bool $enable = null): int|bool
{
}