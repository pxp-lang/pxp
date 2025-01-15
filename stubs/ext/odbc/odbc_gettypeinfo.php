<?php 

/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_gettypeinfo($odbc, int $data_type = 0)
{
}
#[\Since('8.4')]
function odbc_gettypeinfo(\Odbc\Connection $odbc, int $data_type = 0): \Odbc\Result|false
{
}