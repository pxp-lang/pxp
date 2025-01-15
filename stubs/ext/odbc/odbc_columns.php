<?php 

/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_columns($odbc, ?string $catalog = null, ?string $schema = null, ?string $table = null, ?string $column = null)
{
}
#[\Since('8.4')]
function odbc_columns(\Odbc\Connection $odbc, ?string $catalog = null, ?string $schema = null, ?string $table = null, ?string $column = null): \Odbc\Result|false
{
}