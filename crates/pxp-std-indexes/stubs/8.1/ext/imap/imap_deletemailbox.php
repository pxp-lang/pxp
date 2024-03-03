<?php

/** @param resource $imap */
function imap_deletemailbox($imap, string $mailbox): bool
{
}
function imap_deletemailbox(\IMAP\Connection $imap, string $mailbox): bool
{
}