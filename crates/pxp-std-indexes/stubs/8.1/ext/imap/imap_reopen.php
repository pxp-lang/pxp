<?php

/**
 * @param resource $imap
 */
function imap_reopen($imap, string $mailbox, int $flags = 0, int $retries = 0): bool
{
}
function imap_reopen(\IMAP\Connection $imap, string $mailbox, int $flags = 0, int $retries = 0): bool
{
}