<?php

/** @param resource $imap */
function imap_getacl($imap, string $mailbox): array|false
{
}
function imap_getacl(\IMAP\Connection $imap, string $mailbox): array|false
{
}