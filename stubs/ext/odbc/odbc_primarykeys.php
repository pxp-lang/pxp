<?php 

/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_primarykeys($odbc, ?string $catalog, string $schema, string $table)
{
}
#[\Since('8.4')]
function odbc_primarykeys(\Odbc\Connection $odbc, ?string $catalog, string $schema, string $table): \Odbc\Result|false
{
}