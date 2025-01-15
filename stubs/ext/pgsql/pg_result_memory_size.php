<?php 

#ifdef HAVE_PG_RESULT_MEMORY_SIZE
#[\Since('8.4')]
function pg_result_memory_size(\PgSql\Result $result): int
{
}