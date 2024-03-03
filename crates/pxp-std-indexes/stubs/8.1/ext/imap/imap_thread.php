<?php

/** @param resource $imap */
function imap_thread($imap, int $flags = SE_FREE): array|false
{
}
function imap_thread(\IMAP\Connection $imap, int $flags = SE_FREE): array|false
{
}