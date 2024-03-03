<?php

/** @param resource $imap */
function imap_getmailboxes($imap, string $reference, string $pattern): array|false
{
}
function imap_getmailboxes(\IMAP\Connection $imap, string $reference, string $pattern): array|false
{
}