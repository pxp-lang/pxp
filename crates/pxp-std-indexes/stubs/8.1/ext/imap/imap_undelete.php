<?php

/**
 * @param resource $imap
 */
function imap_undelete($imap, string $message_nums, int $flags = 0): bool
{
}
function imap_undelete(\IMAP\Connection $imap, string $message_nums, int $flags = 0): bool
{
}