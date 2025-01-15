<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_free_result($statement): bool
{
}
#[\Since('8.4')]
function odbc_free_result(\Odbc\Result $statement): true
{
}