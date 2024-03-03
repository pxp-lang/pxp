<?php

/** @param resource $imap */
function imap_sort($imap, int $criteria, bool $reverse, int $flags = 0, ?string $search_criteria = null, ?string $charset = null): array|false
{
}
function imap_sort(\IMAP\Connection $imap, int $criteria, bool $reverse, int $flags = 0, ?string $search_criteria = null, ?string $charset = null): array|false
{
}