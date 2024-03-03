<?php

#if defined(HAVE_IMAP2000) || defined(HAVE_IMAP2001)
function imap_get_quota(\IMAP\Connection $imap, string $quota_root): array|false
{
}