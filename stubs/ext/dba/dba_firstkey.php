<?php 

/** @param resource $dba */
#[\Until('8.4')]
function dba_firstkey($dba): string|false
{
}
#[\Since('8.4')]
function dba_firstkey(\Dba\Connection $dba): string|false
{
}