<?php

/**
 * @param resource $imap
 * @alias imap_renamemailbox
 */
function imap_rename($imap, string $from, string $to): bool
{
}
/** @alias imap_renamemailbox */
function imap_rename(\IMAP\Connection $imap, string $from, string $to): bool
{
}