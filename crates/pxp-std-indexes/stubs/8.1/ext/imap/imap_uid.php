<?php

/** @param resource $imap */
function imap_uid($imap, int $message_num): int|false
{
}
function imap_uid(\IMAP\Connection $imap, int $message_num): int|false
{
}