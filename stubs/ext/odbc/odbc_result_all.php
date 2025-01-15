<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_result_all($statement, string $format = ""): int|false
{
}
#[\Deprecated(since: '8.1')]
#[\Since('8.4')]
function odbc_result_all(\Odbc\Result $statement, string $format = ""): int|false
{
}