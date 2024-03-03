<?php

/** @param resource $imap */
function imap_setacl($imap, string $mailbox, string $user_id, string $rights): bool
{
}
function imap_setacl(\IMAP\Connection $imap, string $mailbox, string $user_id, string $rights): bool
{
}