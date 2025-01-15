<?php 

/**
 * @param resource $odbc
 * @return resource|false
 * @alias odbc_exec
 */
#[\Until('8.4')]
function odbc_do($odbc, string $query)
{
}
/** @alias odbc_exec */
#[\Since('8.4')]
function odbc_do(\Odbc\Connection $odbc, string $query): \Odbc\Result|false
{
}