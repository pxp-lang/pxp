<?php

/**
 * @param resource $imap
 * @alias imap_list
 */
function imap_listmailbox($imap, string $reference, string $pattern): array|false
{
}
/** @alias imap_list */
function imap_listmailbox(\IMAP\Connection $imap, string $reference, string $pattern): array|false
{
}