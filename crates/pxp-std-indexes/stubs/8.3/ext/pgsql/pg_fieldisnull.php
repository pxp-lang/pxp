<?php

/**
 * @param string|int $row
 * @alias pg_field_is_null
 * @deprecated
 */
function pg_fieldisnull(\PgSql\Result $result, $row, string|int $field = UNKNOWN): int|false
{
}