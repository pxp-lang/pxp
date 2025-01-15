<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_result($statement, string|int $field): string|bool|null
{
}
#[\Since('8.4')]
function odbc_result(\Odbc\Result $statement, string|int $field): string|bool|null
{
}