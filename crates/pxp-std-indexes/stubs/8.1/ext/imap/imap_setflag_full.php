<?php

/** @param resource $imap */
function imap_setflag_full($imap, string $sequence, string $flag, int $options = 0): bool
{
}
function imap_setflag_full(\IMAP\Connection $imap, string $sequence, string $flag, int $options = 0): bool
{
}