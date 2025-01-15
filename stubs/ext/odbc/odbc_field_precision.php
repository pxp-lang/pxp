<?php 

/**
 * @param resource $statement
 * @alias odbc_field_len
 */
#[\Until('8.4')]
function odbc_field_precision($statement, int $field): int|false
{
}
/** @alias odbc_field_len */
#[\Since('8.4')]
function odbc_field_precision(\Odbc\Result $statement, int $field): int|false
{
}