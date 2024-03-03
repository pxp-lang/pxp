<?php

#ifdef HAVE_PG_CONTEXT_VISIBILITY
function pg_set_error_context_visibility(\PgSql\Connection $connection, int $visibility): int
{
}