<?php 

/** @param resource $odbc */
#[\Until('8.4')]
function odbc_setoption($odbc, int $which, int $option, int $value): bool
{
}
#[\Since('8.4')]
function odbc_setoption(\Odbc\Connection|\Odbc\Result $odbc, int $which, int $option, int $value): bool
{
}