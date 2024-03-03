<?php

/**
 * @param resource $imap
 * @alias imap_createmailbox
 */
function imap_create($imap, string $mailbox): bool
{
}
/** @alias imap_createmailbox */
function imap_create(\IMAP\Connection $imap, string $mailbox): bool
{
}