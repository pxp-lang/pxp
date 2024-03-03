<?php

/**
 * @param resource $imap
 */
function imap_delete($imap, string $message_nums, int $flags = 0): bool
{
}
function imap_delete(\IMAP\Connection $imap, string $message_nums, int $flags = 0): bool
{
}