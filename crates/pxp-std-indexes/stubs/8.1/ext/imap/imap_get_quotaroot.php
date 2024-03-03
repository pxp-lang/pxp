<?php

/** @param resource $imap */
function imap_get_quotaroot($imap, string $mailbox): array|false
{
}
function imap_get_quotaroot(\IMAP\Connection $imap, string $mailbox): array|false
{
}