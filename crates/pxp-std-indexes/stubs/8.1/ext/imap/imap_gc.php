<?php

/**
 * @param resource $imap
 */
function imap_gc($imap, int $flags): bool
{
}
function imap_gc(\IMAP\Connection $imap, int $flags): bool
{
}