<?php 

/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_foreignkeys($odbc, ?string $pk_catalog, string $pk_schema, string $pk_table, string $fk_catalog, string $fk_schema, string $fk_table)
{
}
#[\Since('8.4')]
function odbc_foreignkeys(\Odbc\Connection $odbc, ?string $pk_catalog, string $pk_schema, string $pk_table, string $fk_catalog, string $fk_schema, string $fk_table): \Odbc\Result|false
{
}