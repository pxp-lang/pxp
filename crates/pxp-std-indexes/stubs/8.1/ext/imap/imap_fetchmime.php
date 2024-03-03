<?php

/** @param resource $imap */
function imap_fetchmime($imap, int $message_num, string $section, int $flags = 0): string|false
{
}
function imap_fetchmime(\IMAP\Connection $imap, int $message_num, string $section, int $flags = 0): string|false
{
}