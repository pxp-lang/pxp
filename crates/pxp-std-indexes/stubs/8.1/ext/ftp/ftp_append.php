<?php

/** @param resource $ftp */
function ftp_append($ftp, string $remote_filename, string $local_filename, int $mode = FTP_BINARY): bool
{
}
function ftp_append(\FTP\Connection $ftp, string $remote_filename, string $local_filename, int $mode = FTP_BINARY): bool
{
}