<?php 

/** @param resource $odbc */
#[\Until('8.4')]
function odbc_close($odbc): void
{
}
#[\Since('8.4')]
function odbc_close(\Odbc\Connection $odbc): void
{
}