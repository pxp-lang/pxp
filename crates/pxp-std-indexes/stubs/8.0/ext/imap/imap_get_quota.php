<?php

#if defined(HAVE_IMAP2000) || defined(HAVE_IMAP2001)
/** @param resource $imap */
function imap_get_quota($imap, string $quota_root): array|false
{
}