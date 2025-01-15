<?php 

#endif
/**
 * @param resource $statement
 * @param array $array
 */
#[\Until('8.4')]
function odbc_fetch_into($statement, &$array, int $row = 0): int|false
{
}
#endif
/**
 * @param array $array
 */
#[\Since('8.4')]
function odbc_fetch_into(\Odbc\Result $statement, &$array, ?int $row = null): int|false
{
}