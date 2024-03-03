<?php

/**
 * @param resource $imap
 * @alias imap_listscan
 */
function imap_scan($imap, string $reference, string $pattern, string $content): array|false
{
}
/** @alias imap_listscan */
function imap_scan(\IMAP\Connection $imap, string $reference, string $pattern, string $content): array|false
{
}