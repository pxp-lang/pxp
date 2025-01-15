<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_num_rows($statement): int
{
}
#[\Since('8.4')]
function odbc_num_rows(\Odbc\Result $statement): int
{
}