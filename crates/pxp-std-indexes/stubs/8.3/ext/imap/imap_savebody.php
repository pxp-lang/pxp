<?php

/** @param resource|string|int $file */
function imap_savebody(\IMAP\Connection $imap, $file, int $message_num, string $section = "", int $flags = 0): bool
{
}