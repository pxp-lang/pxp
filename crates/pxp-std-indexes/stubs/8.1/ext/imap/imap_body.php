<?php

/** @param resource $imap */
function imap_body($imap, int $message_num, int $flags = 0): string|false
{
}
function imap_body(\IMAP\Connection $imap, int $message_num, int $flags = 0): string|false
{
}