<?php

/**
 * @param resource $imap
 * @param resource|string|int $file
 */
function imap_savebody($imap, $file, int $message_num, string $section = "", int $flags = 0): bool
{
}
/** @param resource|string|int $file */
function imap_savebody(\IMAP\Connection $imap, $file, int $message_num, string $section = "", int $flags = 0): bool
{
}