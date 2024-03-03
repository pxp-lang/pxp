<?php

/** @param resource $imap */
function imap_listscan($imap, string $reference, string $pattern, string $content): array|false
{
}
function imap_listscan(\IMAP\Connection $imap, string $reference, string $pattern, string $content): array|false
{
}