<?php 

/** @param resource $odbc */
#[\Until('8.4')]
function odbc_rollback($odbc): bool
{
}
#[\Since('8.4')]
function odbc_rollback(\Odbc\Connection $odbc): bool
{
}