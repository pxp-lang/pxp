<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_field_name($statement, int $field): string|false
{
}
#[\Since('8.4')]
function odbc_field_name(\Odbc\Result $statement, int $field): string|false
{
}