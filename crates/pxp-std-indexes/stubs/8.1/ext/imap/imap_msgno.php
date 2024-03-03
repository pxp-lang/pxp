<?php

/** @param resource $imap */
function imap_msgno($imap, int $message_uid): int
{
}
function imap_msgno(\IMAP\Connection $imap, int $message_uid): int
{
}