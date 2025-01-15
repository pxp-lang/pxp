<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_cursor($statement): string|false
{
}
#[\Since('8.4')]
function odbc_cursor(\Odbc\Result $statement): string|false
{
}