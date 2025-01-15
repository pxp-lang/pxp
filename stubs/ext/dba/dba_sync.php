<?php 

/** @param resource $dba */
#[\Until('8.4')]
function dba_sync($dba): bool
{
}
#[\Since('8.4')]
function dba_sync(\Dba\Connection $dba): bool
{
}