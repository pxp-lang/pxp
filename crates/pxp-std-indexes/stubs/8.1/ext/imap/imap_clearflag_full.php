<?php

/** @param resource $imap */
function imap_clearflag_full($imap, string $sequence, string $flag, int $options = 0): bool
{
}
function imap_clearflag_full(\IMAP\Connection $imap, string $sequence, string $flag, int $options = 0): bool
{
}