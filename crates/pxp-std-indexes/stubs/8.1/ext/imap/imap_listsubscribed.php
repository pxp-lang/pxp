<?php

/**
 * @param resource $imap
 * @alias imap_lsub
 */
function imap_listsubscribed($imap, string $reference, string $pattern): array|false
{
}
/** @alias imap_lsub */
function imap_listsubscribed(\IMAP\Connection $imap, string $reference, string $pattern): array|false
{
}