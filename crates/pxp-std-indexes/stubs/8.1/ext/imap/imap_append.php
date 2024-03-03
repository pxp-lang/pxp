<?php

/** @param resource $imap */
function imap_append($imap, string $folder, string $message, ?string $options = null, ?string $internal_date = null): bool
{
}
function imap_append(\IMAP\Connection $imap, string $folder, string $message, ?string $options = null, ?string $internal_date = null): bool
{
}