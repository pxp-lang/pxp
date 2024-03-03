<?php

/**
 * @param resource $imap
 * @return stdClass|false
 */
function imap_status($imap, string $mailbox, int $flags)
{
}
function imap_status(\IMAP\Connection $imap, string $mailbox, int $flags): \stdClass|false
{
}