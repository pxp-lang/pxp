<?php

/**
 * @param resource $ftp
 * @param int|bool $value
 */
function ftp_set_option($ftp, int $option, $value): bool
{
}
/** @param int|bool $value */
function ftp_set_option(\FTP\Connection $ftp, int $option, $value): bool
{
}