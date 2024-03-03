<?php

/**
 * @param resource $imap
 * @alias imap_listscan
 */
function imap_scanmailbox($imap, string $reference, string $pattern, string $content): array|false
{
}
/** @alias imap_listscan */
function imap_scanmailbox(\IMAP\Connection $imap, string $reference, string $pattern, string $content): array|false
{
}