<?php

/** @param resource $imap */
function imap_createmailbox($imap, string $mailbox): bool
{
}
function imap_createmailbox(\IMAP\Connection $imap, string $mailbox): bool
{
}