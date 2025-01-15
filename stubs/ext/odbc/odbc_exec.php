<?php 

#endif
/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_exec($odbc, string $query)
{
}
#endif
#[\Since('8.4')]
function odbc_exec(\Odbc\Connection $odbc, string $query): \Odbc\Result|false
{
}