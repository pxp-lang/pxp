<?php

/** @param resource $imap */
function imap_num_msg($imap): int|false
{
}
function imap_num_msg(\IMAP\Connection $imap): int|false
{
}