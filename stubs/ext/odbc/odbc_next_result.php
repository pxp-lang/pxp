<?php 

#if !defined(HAVE_SOLID) && !defined(HAVE_SOLID_30)
/** @param resource $statement */
#[\Until('8.4')]
function odbc_next_result($statement): bool
{
}
#if !defined(HAVE_SOLID) && !defined(HAVE_SOLID_30)
#[\Since('8.4')]
function odbc_next_result(\Odbc\Result $statement): bool
{
}