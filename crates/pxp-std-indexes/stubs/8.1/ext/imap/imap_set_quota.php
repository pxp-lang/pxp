<?php

/** @param resource $imap */
function imap_set_quota($imap, string $quota_root, int $mailbox_size): bool
{
}
function imap_set_quota(\IMAP\Connection $imap, string $quota_root, int $mailbox_size): bool
{
}