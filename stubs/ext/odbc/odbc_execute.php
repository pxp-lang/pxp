<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_execute($statement, array $params = []): bool
{
}
#[\Since('8.4')]
function odbc_execute(\Odbc\Result $statement, array $params = []): bool
{
}