<?php

/** @param resource $stream */
function ftp_nb_fget(\FTP\Connection $ftp, $stream, string $remote_filename, int $mode = FTP_BINARY, int $offset = 0): int
{
}