<?php

/** @param resource $imap */
function imap_fetchstructure($imap, int $message_num, int $flags = 0): \stdClass|false
{
}
function imap_fetchstructure(\IMAP\Connection $imap, int $message_num, int $flags = 0): \stdClass|false
{
}