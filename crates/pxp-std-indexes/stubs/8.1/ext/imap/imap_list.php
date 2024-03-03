<?php

/** @param resource $imap */
function imap_list($imap, string $reference, string $pattern): array|false
{
}
function imap_list(\IMAP\Connection $imap, string $reference, string $pattern): array|false
{
}