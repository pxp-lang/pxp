<?php 

#if !defined(HAVE_DBMAKER) && !defined(HAVE_SOLID) && !defined(HAVE_SOLID_30) &&!defined(HAVE_SOLID_35)
/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_tableprivileges($odbc, ?string $catalog, string $schema, string $table)
{
}
#if !defined(HAVE_DBMAKER) && !defined(HAVE_SOLID) && !defined(HAVE_SOLID_30) &&!defined(HAVE_SOLID_35)
#[\Since('8.4')]
function odbc_tableprivileges(\Odbc\Connection $odbc, ?string $catalog, string $schema, string $table): \Odbc\Result|false
{
}