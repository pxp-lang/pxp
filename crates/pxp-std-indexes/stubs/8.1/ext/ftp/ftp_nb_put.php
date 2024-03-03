<?php

/** @param resource $ftp */
function ftp_nb_put($ftp, string $remote_filename, string $local_filename, int $mode = FTP_BINARY, int $offset = 0): int|false
{
}
function ftp_nb_put(\FTP\Connection $ftp, string $remote_filename, string $local_filename, int $mode = FTP_BINARY, int $offset = 0): int|false
{
}