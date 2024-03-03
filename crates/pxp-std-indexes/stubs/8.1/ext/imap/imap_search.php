<?php

/** @param resource $imap */
function imap_search($imap, string $criteria, int $flags = SE_FREE, string $charset = ""): array|false
{
}
function imap_search(\IMAP\Connection $imap, string $criteria, int $flags = SE_FREE, string $charset = ""): array|false
{
}