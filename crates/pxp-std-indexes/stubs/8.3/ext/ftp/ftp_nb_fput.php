<?php

/** @param resource $stream */
function ftp_nb_fput(\FTP\Connection $ftp, string $remote_filename, $stream, int $mode = FTP_BINARY, int $offset = 0): int
{
}