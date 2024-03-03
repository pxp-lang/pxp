<?php

/** @param resource $imap */
function imap_renamemailbox($imap, string $from, string $to): bool
{
}
function imap_renamemailbox(\IMAP\Connection $imap, string $from, string $to): bool
{
}