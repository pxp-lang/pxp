<?php

/**
 * @param resource $imap
 */
function imap_close($imap, int $flags = 0): bool
{
}
function imap_close(\IMAP\Connection $imap, int $flags = 0): bool
{
}