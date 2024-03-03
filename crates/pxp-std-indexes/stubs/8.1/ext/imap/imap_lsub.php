<?php

/** @param resource $imap */
function imap_lsub($imap, string $reference, string $pattern): array|false
{
}
function imap_lsub(\IMAP\Connection $imap, string $reference, string $pattern): array|false
{
}