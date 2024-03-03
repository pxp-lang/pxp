<?php

/** @param resource $imap */
function imap_fetch_overview($imap, string $sequence, int $flags = 0): array|false
{
}
function imap_fetch_overview(\IMAP\Connection $imap, string $sequence, int $flags = 0): array|false
{
}