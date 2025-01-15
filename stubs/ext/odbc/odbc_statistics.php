<?php 

/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_statistics($odbc, ?string $catalog, string $schema, string $table, int $unique, int $accuracy)
{
}
#[\Since('8.4')]
function odbc_statistics(\Odbc\Connection $odbc, ?string $catalog, string $schema, string $table, int $unique, int $accuracy): \Odbc\Result|false
{
}