<?php

/** @param resource $imap */
function imap_unsubscribe($imap, string $mailbox): bool
{
}
function imap_unsubscribe(\IMAP\Connection $imap, string $mailbox): bool
{
}