<?php

/** @param resource $imap */
function imap_subscribe($imap, string $mailbox): bool
{
}
function imap_subscribe(\IMAP\Connection $imap, string $mailbox): bool
{
}