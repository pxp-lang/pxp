<?php 

/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_tables($odbc, ?string $catalog = null, ?string $schema = null, ?string $table = null, ?string $types = null)
{
}
#[\Since('8.4')]
function odbc_tables(\Odbc\Connection $odbc, ?string $catalog = null, ?string $schema = null, ?string $table = null, ?string $types = null): \Odbc\Result|false
{
}