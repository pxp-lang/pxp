<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_field_scale($statement, int $field): int|false
{
}
#[\Since('8.4')]
function odbc_field_scale(\Odbc\Result $statement, int $field): int|false
{
}