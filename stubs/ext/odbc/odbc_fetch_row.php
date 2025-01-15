<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_fetch_row($statement, ?int $row = null): bool
{
}
#[\Since('8.4')]
function odbc_fetch_row(\Odbc\Result $statement, ?int $row = null): bool
{
}