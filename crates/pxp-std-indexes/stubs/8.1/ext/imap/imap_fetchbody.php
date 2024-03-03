<?php

/** @param resource $imap */
function imap_fetchbody($imap, int $message_num, string $section, int $flags = 0): string|false
{
}
function imap_fetchbody(\IMAP\Connection $imap, int $message_num, string $section, int $flags = 0): string|false
{
}