<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_longreadlen($statement, int $length): bool
{
}
#[\Since('8.4')]
function odbc_longreadlen(\Odbc\Result $statement, int $length): true
{
}