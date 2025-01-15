<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_field_type($statement, int $field): string|false
{
}
#[\Since('8.4')]
function odbc_field_type(\Odbc\Result $statement, int $field): string|false
{
}