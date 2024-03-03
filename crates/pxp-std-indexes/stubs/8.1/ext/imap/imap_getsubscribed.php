<?php

/** @param resource $imap */
function imap_getsubscribed($imap, string $reference, string $pattern): array|false
{
}
function imap_getsubscribed(\IMAP\Connection $imap, string $reference, string $pattern): array|false
{
}