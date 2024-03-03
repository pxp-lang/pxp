<?php

/** @param resource $imap */
function imap_headerinfo($imap, int $message_num, int $from_length = 0, int $subject_length = 0): \stdClass|false
{
}
function imap_headerinfo(\IMAP\Connection $imap, int $message_num, int $from_length = 0, int $subject_length = 0): \stdClass|false
{
}