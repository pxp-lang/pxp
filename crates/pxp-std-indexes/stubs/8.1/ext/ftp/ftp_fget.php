<?php

/**
 * @param resource $ftp
 * @param resource $stream
 */
function ftp_fget($ftp, $stream, string $remote_filename, int $mode = FTP_BINARY, int $offset = 0): bool
{
}
/** @param resource $stream */
function ftp_fget(\FTP\Connection $ftp, $stream, string $remote_filename, int $mode = FTP_BINARY, int $offset = 0): bool
{
}