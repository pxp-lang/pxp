<?php

/** @param resource $imap */
function imap_mail_move($imap, string $message_nums, string $mailbox, int $flags = 0): bool
{
}
function imap_mail_move(\IMAP\Connection $imap, string $message_nums, string $mailbox, int $flags = 0): bool
{
}