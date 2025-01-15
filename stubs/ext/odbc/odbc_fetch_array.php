<?php 

/** @param resource $statement */
#[\Until('8.4')]
function odbc_fetch_array($statement, int $row = -1): array|false
{
}
#[\Since('8.4')]
function odbc_fetch_array(\Odbc\Result $statement, ?int $row = null): array|false
{
}