<?php

/**
 * @param resource $imap
 * @return stdClass|false
 */
function imap_bodystruct($imap, int $message_num, string $section)
{
}
function imap_bodystruct(\IMAP\Connection $imap, int $message_num, string $section): \stdClass|false
{
}