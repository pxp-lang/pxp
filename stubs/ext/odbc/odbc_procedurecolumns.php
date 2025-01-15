<?php 

#if !defined(HAVE_SOLID) && !defined(HAVE_SOLID_30) && !defined(HAVE_SOLID_35)
/**
 * @param resource $odbc
 * @return resource|false
 */
#[\Until('8.4')]
function odbc_procedurecolumns($odbc, ?string $catalog = null, ?string $schema = null, ?string $procedure = null, ?string $column = null)
{
}
#if !defined(HAVE_SOLID) && !defined(HAVE_SOLID_30) && !defined(HAVE_SOLID_35)
#[\Since('8.4')]
function odbc_procedurecolumns(\Odbc\Connection $odbc, ?string $catalog = null, ?string $schema = null, ?string $procedure = null, ?string $column = null): \Odbc\Result|false
{
}