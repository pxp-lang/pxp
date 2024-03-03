<?php

/** @alias imap_body */
function imap_fetchtext(\IMAP\Connection $imap, int $message_num, int $flags = 0): string|false
{
}