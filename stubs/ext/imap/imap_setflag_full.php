<?php 

/** @param resource $imap */
#[\Until('8.1')]
function imap_setflag_full($imap, string $sequence, string $flag, int $options = 0): bool
{
}
#[\Since('8.1')]
#[\Until('8.3')]
function imap_setflag_full(\IMAP\Connection $imap, string $sequence, string $flag, int $options = 0): bool
{
}
#[\Since('8.3')]
function imap_setflag_full(\IMAP\Connection $imap, string $sequence, string $flag, int $options = 0): true
{
}