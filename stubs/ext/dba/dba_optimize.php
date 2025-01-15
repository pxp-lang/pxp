<?php 

/** @param resource $dba */
#[\Until('8.4')]
function dba_optimize($dba): bool
{
}
#[\Since('8.4')]
function dba_optimize(\Dba\Connection $dba): bool
{
}